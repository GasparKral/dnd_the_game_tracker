package io.github.gasparkral.dnd.ui.screen

import androidx.compose.animation.AnimatedVisibility
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material.icons.filled.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontStyle
import androidx.compose.ui.unit.dp
import android.text.method.LinkMovementMethod
import android.widget.TextView
import androidx.compose.ui.graphics.toArgb
import androidx.compose.ui.viewinterop.AndroidView
import androidx.core.text.HtmlCompat
import io.github.gasparkral.dnd.infra.HttpError
import io.github.gasparkral.dnd.infra.HttpManager
import kotlinx.serialization.Serializable
import org.intellij.markdown.flavours.commonmark.CommonMarkFlavourDescriptor
import org.intellij.markdown.html.HtmlGenerator
import org.intellij.markdown.parser.MarkdownParser


// ── Modelos ──────────────────────────────────────────────────────────────────

@Serializable
data class LoreIndexEntry(val path: String, val title: String)

@Serializable
private data class LoreIndexResponse(val entries: List<LoreIndexEntry>)

@Serializable
data class LoreEntry(val path: String, val title: String, val content: String)

// ── Normalización del contenido ───────────────────────────────────────────────

/**
 * El vault de Obsidian mezcla etiquetas HTML (<h3>) con Markdown puro y
 * wikilinks ([[Nombre]]). Esta función lo normaliza a Markdown estándar
 * para que la librería de renderizado lo procese correctamente.
 */
fun normalizeObsidianContent(raw: String): String = raw
    // Wikilinks [[Título]] → texto en negrita **Título**
    .replace(Regex("""\[\[([^\]|]+)(?:\|[^\]]*)?\]\]""")) { "**${it.groupValues[1]}**" }
    // <h1>…</h1> → # …
    .replace(Regex("""<h1[^>]*>(.*?)</h1>""", RegexOption.DOT_MATCHES_ALL)) { "# ${it.groupValues[1].trim()}" }
    // <h2>…</h2> → ## …
    .replace(Regex("""<h2[^>]*>(.*?)</h2>""", RegexOption.DOT_MATCHES_ALL)) { "## ${it.groupValues[1].trim()}" }
    // <h3>…</h3> → ### …
    .replace(Regex("""<h3[^>]*>(.*?)</h3>""", RegexOption.DOT_MATCHES_ALL)) { "### ${it.groupValues[1].trim()}" }
    // <h4>…</h4> → #### …
    .replace(Regex("""<h4[^>]*>(.*?)</h4>""", RegexOption.DOT_MATCHES_ALL)) { "#### ${it.groupValues[1].trim()}" }
    // Cualquier otra etiqueta HTML restante → vacío
    .replace(Regex("""<[^>]+>"""), "")
    // Colapsar más de dos saltos de línea seguidos
    .replace(Regex("""\n{3,}"""), "\n\n")
    .trim()

// ── Árbol de nodos ────────────────────────────────────────────────────────────

sealed class LoreNode {
    data class Folder(val name: String, val depth: Int, val children: List<LoreNode>) : LoreNode()
    data class Leaf(val entry: LoreIndexEntry, val depth: Int) : LoreNode()
}

fun buildTree(entries: List<LoreIndexEntry>): List<LoreNode> {
    val root = mutableMapOf<String, MutableList<LoreIndexEntry>>()

    for (entry in entries) {
        val parts = entry.path.split("/")
        val folder = if (parts.size > 2) parts.drop(1).dropLast(1).joinToString("/") else ""
        root.getOrPut(folder) { mutableListOf() }.add(entry)
    }

    fun buildLevel(prefix: String, depth: Int): List<LoreNode> {
        val directChildren = root[prefix] ?: emptyList()
        val subFolders = root.keys
            .filter { key ->
                if (prefix.isEmpty()) !key.contains("/") && key.isNotEmpty()
                else key.startsWith("$prefix/") && !key.removePrefix("$prefix/").contains("/")
            }
            .sorted()

        val folderNodes = subFolders.map { folderKey ->
            LoreNode.Folder(
                name = folderKey.substringAfterLast("/").ifEmpty { folderKey },
                depth = depth,
                children = buildLevel(folderKey, depth + 1)
            )
        }
        val leafNodes = directChildren
            .sortedBy { it.title }
            .map { LoreNode.Leaf(it, depth) }

        return folderNodes + leafNodes
    }

    return buildLevel("", 0)
}

// ── Pantalla ─────────────────────────────────────────────────────────────────

@Composable
fun LoreScreen(
    modifier: Modifier = Modifier,
    onBack: () -> Unit = {}
) {
    var index by remember { mutableStateOf<List<LoreIndexEntry>>(emptyList()) }
    var indexError by remember { mutableStateOf<String?>(null) }
    var query by remember { mutableStateOf("") }

    LaunchedEffect(Unit) {
        HttpManager.get<LoreIndexResponse>("/api/lore").fold(
            onOk = { index = it.entries },
            onErr = { e -> indexError = e.toString() }
        )
    }

    val tree by remember(index) { derivedStateOf { buildTree(index) } }

    val isFiltering = query.isNotBlank()
    val filteredFlat by remember(index, query) {
        derivedStateOf {
            if (isFiltering) index.filter { it.title.contains(query, ignoreCase = true) }.sortedBy { it.title }
            else emptyList()
        }
    }

    Column(modifier.padding(horizontal = 16.dp, vertical = 8.dp)) {

        // ── Cabecera ──────────────────────────────────────────────────────────
        Row(
            modifier = Modifier.fillMaxWidth(),
            verticalAlignment = Alignment.CenterVertically
        ) {
            IconButton(onClick = onBack) {
                Icon(Icons.AutoMirrored.Filled.ArrowBack, contentDescription = "Volver")
            }
            Icon(Icons.Filled.Book, contentDescription = null, modifier = Modifier.size(20.dp))
            Spacer(Modifier.width(8.dp))
            Text("Lore del mundo", style = MaterialTheme.typography.headlineSmall)
        }

        Spacer(Modifier.height(8.dp))

        // ── Barra de búsqueda ─────────────────────────────────────────────────
        OutlinedTextField(
            value = query,
            onValueChange = { query = it },
            placeholder = { Text("Buscar…") },
            leadingIcon = { Icon(Icons.Filled.Search, contentDescription = null) },
            trailingIcon = {
                if (query.isNotEmpty()) {
                    IconButton(onClick = { query = "" }) {
                        Icon(Icons.Filled.Clear, contentDescription = "Limpiar")
                    }
                }
            },
            singleLine = true,
            shape = RoundedCornerShape(12.dp),
            modifier = Modifier.fillMaxWidth()
        )

        Spacer(Modifier.height(12.dp))

        // ── Cuerpo ────────────────────────────────────────────────────────────
        when {
            indexError != null -> {
                Box(Modifier.fillMaxSize(), contentAlignment = Alignment.Center) {
                    Text(
                        "⚠ No se pudo cargar el lore\n$indexError",
                        style = MaterialTheme.typography.bodyMedium,
                        color = MaterialTheme.colorScheme.error
                    )
                }
            }

            index.isEmpty() -> {
                Box(Modifier.fillMaxSize(), contentAlignment = Alignment.Center) {
                    Text("El maestro aún no ha revelado nada…")
                }
            }

            isFiltering -> {
                if (filteredFlat.isEmpty()) {
                    Box(Modifier.fillMaxSize(), contentAlignment = Alignment.Center) {
                        Text(
                            "Sin resultados para \"$query\"",
                            fontStyle = FontStyle.Italic,
                            color = MaterialTheme.colorScheme.onSurfaceVariant
                        )
                    }
                } else {
                    LazyColumn(verticalArrangement = Arrangement.spacedBy(6.dp)) {
                        items(filteredFlat, key = { it.path }) { entry ->
                            LoreLeafCard(entry = entry, depth = 0, showPath = true)
                        }
                    }
                }
            }

            else -> {
                LazyColumn(verticalArrangement = Arrangement.spacedBy(4.dp)) {
                    items(tree, key = { node ->
                        when (node) {
                            is LoreNode.Folder -> "folder:${node.name}"
                            is LoreNode.Leaf -> "leaf:${node.entry.path}"
                        }
                    }) { node ->
                        LoreNodeItem(node)
                    }
                }
            }
        }
    }
}

// ── Renderer Markdown → HTML → TextView ──────────────────────────────────────

@Composable
fun MarkdownView(markdown: String, modifier: Modifier = Modifier) {
    val textColor = MaterialTheme.colorScheme.onSurface.toArgb()

    val html = remember(markdown) {
        val flavour = CommonMarkFlavourDescriptor()
        val parsedTree = MarkdownParser(flavour).buildMarkdownTreeFromString(markdown)
        HtmlGenerator(markdown, parsedTree, flavour).generateHtml()
    }

    AndroidView(
        modifier = modifier.fillMaxWidth(),
        factory = { context ->
            TextView(context).apply {
                movementMethod = LinkMovementMethod.getInstance()
                setTextColor(textColor)
            }
        },
        update = { view ->
            view.setTextColor(textColor)
            view.text = HtmlCompat.fromHtml(html, HtmlCompat.FROM_HTML_MODE_COMPACT)
        }
    )
}

// ── Nodo del árbol ────────────────────────────────────────────────────────────

@Composable
fun LoreNodeItem(node: LoreNode) {
    when (node) {
        is LoreNode.Folder -> LoreFolderItem(node)
        is LoreNode.Leaf -> LoreLeafCard(entry = node.entry, depth = node.depth)
    }
}

@Composable
fun LoreFolderItem(folder: LoreNode.Folder) {
    var expanded by remember { mutableStateOf(true) }
    val indent = (folder.depth * 12).dp

    Column {
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .clickable { expanded = !expanded }
                .padding(start = indent + 4.dp, top = 6.dp, bottom = 6.dp, end = 8.dp),
            verticalAlignment = Alignment.CenterVertically,
            horizontalArrangement = Arrangement.spacedBy(6.dp)
        ) {
            Icon(
                imageVector = if (expanded) Icons.Filled.KeyboardArrowDown
                else Icons.Filled.KeyboardArrowRight,
                contentDescription = null,
                modifier = Modifier.size(18.dp),
                tint = MaterialTheme.colorScheme.primary
            )
            Icon(
                imageVector = if (expanded) Icons.Filled.FolderOpen else Icons.Filled.Folder,
                contentDescription = null,
                modifier = Modifier.size(18.dp),
                tint = MaterialTheme.colorScheme.primary
            )
            Text(
                text = folder.name,
                style = MaterialTheme.typography.titleSmall,
                color = MaterialTheme.colorScheme.primary
            )
        }

        AnimatedVisibility(visible = expanded) {
            Column {
                folder.children.forEach { child -> LoreNodeItem(child) }
            }
        }
    }
}

// ── Card de hoja con lazy load y render Markdown ──────────────────────────────

@Composable
fun LoreLeafCard(
    entry: LoreIndexEntry,
    depth: Int,
    showPath: Boolean = false,
) {
    var expanded by remember(entry.path) { mutableStateOf(false) }
    var content by remember(entry.path) { mutableStateOf<String?>(null) }
    var loading by remember(entry.path) { mutableStateOf(false) }
    var loadError by remember(entry.path) { mutableStateOf<HttpError?>(null) }
    val indent = (depth * 12 + 8).dp

    LaunchedEffect(expanded) {
        if (expanded && content == null && !loading) {
            loading = true
            loadError = null
            val endpoint = "/api/lore/" + entry.path.removeSuffix(".md")
            HttpManager.get<LoreEntry>(endpoint).fold(
                onOk = { content = normalizeObsidianContent(it.content) },
                onErr = { loadError = it }
            )
            loading = false
        }
    }

    Card(
        onClick = { expanded = !expanded },
        modifier = Modifier
            .fillMaxWidth()
            .padding(start = indent)
    ) {
        Column(Modifier.padding(horizontal = 12.dp, vertical = 10.dp)) {

            // ── Cabecera de la card ───────────────────────────────────────────
            Row(
                verticalAlignment = Alignment.Top,  // Top para títulos multilinea
                modifier = Modifier.fillMaxWidth()
            ) {
                Icon(
                    imageVector = Icons.Filled.Description,
                    contentDescription = null,
                    modifier = Modifier
                        .size(14.dp)
                        .padding(top = 2.dp),  // alinear con la primera línea del texto
                    tint = MaterialTheme.colorScheme.onSurfaceVariant
                )
                Spacer(Modifier.width(6.dp))
                Column(modifier = Modifier.weight(1f)) {  // weight evita overflow horizontal
                    Text(
                        text = entry.title,
                        style = MaterialTheme.typography.titleSmall,
                        maxLines = 3,
                        overflow = androidx.compose.ui.text.style.TextOverflow.Ellipsis
                    )
                    if (showPath) {
                        val folder = entry.path
                            .split("/")
                            .drop(1)
                            .dropLast(1)
                            .joinToString(" › ")
                        if (folder.isNotEmpty()) {
                            Text(
                                text = folder,
                                style = MaterialTheme.typography.labelSmall,
                                color = MaterialTheme.colorScheme.onSurfaceVariant,
                                fontStyle = FontStyle.Italic
                            )
                        }
                    }
                }
            }

            // ── Contenido expandible ──────────────────────────────────────────
            AnimatedVisibility(visible = expanded) {
                Column {
                    Spacer(Modifier.height(8.dp))
                    HorizontalDivider()
                    Spacer(Modifier.height(8.dp))
                    when {
                        loading -> CircularProgressIndicator(
                            modifier = Modifier
                                .size(20.dp)
                                .align(Alignment.CenterHorizontally),
                            strokeWidth = 2.dp
                        )

                        loadError != null -> Text(
                            "⚠ $loadError",
                            style = MaterialTheme.typography.bodySmall,
                            color = MaterialTheme.colorScheme.error
                        )

                        content != null -> MarkdownView(markdown = content!!)
                    }
                }
            }
        }
    }
}
