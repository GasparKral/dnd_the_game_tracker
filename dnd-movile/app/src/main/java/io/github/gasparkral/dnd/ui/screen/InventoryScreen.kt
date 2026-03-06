package io.github.gasparkral.dnd.ui.screen


import androidx.compose.animation.AnimatedVisibility
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.LazyRow
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material.icons.filled.Add
import androidx.compose.material.icons.filled.Delete
import androidx.compose.material.icons.filled.MonetizationOn
import androidx.compose.material.icons.filled.Remove
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import io.github.gasparkral.dnd.model.Currency
import io.github.gasparkral.dnd.model.InventoryItem
import io.github.gasparkral.dnd.model.ItemCategory
import io.github.gasparkral.dnd.ui.theme.*
import io.github.gasparkral.dnd.ui.viewmodel.InventoryViewModel
import org.koin.androidx.compose.koinViewModel
import org.koin.core.parameter.parametersOf

// ---------------------------------------------------------------------------
// Pantalla principal
// ---------------------------------------------------------------------------

@Composable
fun InventoryScreen(
    modifier: Modifier = Modifier,
    draftId: String,
    onBack: () -> Unit = {},
) {
    val vm: InventoryViewModel = koinViewModel(parameters = { parametersOf(draftId) })
    val state by vm.state.collectAsState()

    // Filtro de categoría activo (null = todos)
    var activeFilter by remember { mutableStateOf<ItemCategory?>(null) }

    val visibleItems = remember(state.items, activeFilter) {
        if (activeFilter == null) state.items
        else state.items.filter { it.category == activeFilter }
    }

    Box(modifier.background(Parchment)) {
        Column(Modifier.fillMaxSize()) {

            // ── Cabecera ──────────────────────────────────────────────────
            InventoryHeader(
                itemCount = state.items.size,
                totalWeight = state.totalWeight,
                onBack = onBack,
                onAddClick = vm::openAddDialog,
                onCurrencyClick = vm::openCurrencyDialog,
            )

            when {
                state.isLoading -> Box(
                    Modifier.fillMaxSize(),
                    contentAlignment = Alignment.Center
                ) { CircularProgressIndicator(color = Aurum) }

                state.error != null -> Box(
                    Modifier.fillMaxSize(),
                    contentAlignment = Alignment.Center
                ) {
                    Text(
                        "⚠ ${state.error}",
                        color = Ember,
                        style = MaterialTheme.typography.bodyMedium,
                    )
                }

                else -> {
                    // ── Monedas ───────────────────────────────────────────
                    CurrencyBar(
                        currency = state.currency,
                        modifier = Modifier.padding(horizontal = 16.dp, vertical = 8.dp),
                    )

                    // ── Filtros por categoría ─────────────────────────────
                    CategoryFilterRow(
                        active = activeFilter,
                        onSelect = { activeFilter = if (activeFilter == it) null else it },
                        modifier = Modifier.padding(horizontal = 16.dp),
                    )

                    Spacer(Modifier.height(8.dp))

                    // ── Lista de objetos ──────────────────────────────────
                    if (visibleItems.isEmpty()) {
                        Box(
                            Modifier
                                .fillMaxSize()
                                .padding(32.dp),
                            contentAlignment = Alignment.Center,
                        ) {
                            Text(
                                text = if (activeFilter == null) "El morral está vacío." else "Sin objetos de tipo ${activeFilter!!.name}.",
                                style = MaterialTheme.typography.bodyMedium,
                                color = Ash,
                            )
                        }
                    } else {
                        LazyColumn(
                            contentPadding = PaddingValues(horizontal = 16.dp, vertical = 4.dp),
                            verticalArrangement = Arrangement.spacedBy(8.dp),
                        ) {
                            items(visibleItems, key = { it.id }) { item ->
                                ItemCard(
                                    item = item,
                                    onToggleEquipped = { vm.toggleEquipped(item) },
                                    onIncrement = { vm.updateQuantity(item, item.quantity + 1) },
                                    onDecrement = { vm.updateQuantity(item, item.quantity - 1) },
                                    onDelete = { vm.deleteItem(item) },
                                )
                            }
                            item { Spacer(Modifier.height(80.dp)) }
                        }
                    }
                }
            }
        }

        // ── FAB añadir ────────────────────────────────────────────────────
        if (!state.isLoading && state.error == null) {
            FloatingActionButton(
                onClick = vm::openAddDialog,
                modifier = Modifier
                    .align(Alignment.BottomEnd)
                    .padding(20.dp),
                containerColor = Aurum,
            ) {
                Icon(Icons.Filled.Add, contentDescription = "Añadir objeto")
            }
        }
    }

    // ── Diálogos ─────────────────────────────────────────────────────────────
    if (state.showAddDialog) {
        AddItemDialog(
            isSaving = state.isSaving,
            onDismiss = vm::closeAddDialog,
            onConfirm = { name, cat, desc, qty, weight, notes ->
                vm.addItem(name, cat, desc, qty, weight, notes)
            },
        )
    }

    if (state.showCurrencyDialog) {
        CurrencyDialog(
            current = state.currency,
            isSaving = state.isSaving,
            onDismiss = vm::closeCurrencyDialog,
            onConfirm = { cp, sp, ep, gp, pp ->
                vm.updateCurrency(cp, sp, ep, gp, pp)
            },
        )
    }
}

// ---------------------------------------------------------------------------
// Cabecera
// ---------------------------------------------------------------------------

@Composable
private fun InventoryHeader(
    itemCount: Int,
    totalWeight: Float,
    onBack: () -> Unit,
    onAddClick: () -> Unit,
    onCurrencyClick: () -> Unit,
) {
    Surface(color = Crypt, shadowElevation = 4.dp) {
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(horizontal = 4.dp, vertical = 4.dp),
            verticalAlignment = Alignment.CenterVertically,
        ) {
            IconButton(onClick = onBack) {
                Icon(
                    Icons.AutoMirrored.Filled.ArrowBack,
                    contentDescription = "Volver",
                    tint = Aurum,
                )
            }
            Column(Modifier.weight(1f)) {
                Text("Inventario", style = MaterialTheme.typography.titleLarge)
                Text(
                    "$itemCount objetos · %.1f lb".format(totalWeight),
                    style = MaterialTheme.typography.bodySmall,
                    color = Ash,
                )
            }
            IconButton(onClick = onCurrencyClick) {
                Icon(Icons.Filled.MonetizationOn, contentDescription = "Monedas", tint = Gold)
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Barra de monedas
// ---------------------------------------------------------------------------

@Composable
private fun CurrencyBar(currency: Currency, modifier: Modifier = Modifier) {
    Card(
        modifier = modifier.fillMaxWidth(),
        colors = CardDefaults.cardColors(containerColor = Crypt),
        shape = RoundedCornerShape(8.dp),
    ) {
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(horizontal = 12.dp, vertical = 8.dp),
            horizontalArrangement = Arrangement.SpaceEvenly,
        ) {
            CoinChip("PP", currency.platinum, Gold)
            CoinChip("PO", currency.gold, Gold)
            CoinChip("PE", currency.electrum, Ash)
            CoinChip("PA", currency.silver, Ash)
            CoinChip("PC", currency.copper, Ember)
        }
    }
}

@Composable
private fun CoinChip(label: String, amount: Int, tint: androidx.compose.ui.graphics.Color) {
    Column(horizontalAlignment = Alignment.CenterHorizontally) {
        Text(
            amount.toString(),
            style = MaterialTheme.typography.titleSmall,
            color = if (amount > 0) tint else Ash,
        )
        Text(label, style = MaterialTheme.typography.labelSmall, color = Ash)
    }
}

// ---------------------------------------------------------------------------
// Filtros de categoría
// ---------------------------------------------------------------------------

@Composable
private fun CategoryFilterRow(
    active: ItemCategory?,
    onSelect: (ItemCategory) -> Unit,
    modifier: Modifier = Modifier,
) {
    LazyRow(
        modifier = modifier,
        horizontalArrangement = Arrangement.spacedBy(8.dp),
    ) {
        items(ItemCategory.entries) { cat ->
            FilterChip(
                selected = active == cat,
                onClick = { onSelect(cat) },
                label = { Text("${cat.emoji()} ${cat.label()}") },
            )
        }
    }
}

// ---------------------------------------------------------------------------
// Card de objeto
// ---------------------------------------------------------------------------

@Composable
private fun ItemCard(
    item: InventoryItem,
    onToggleEquipped: () -> Unit,
    onIncrement: () -> Unit,
    onDecrement: () -> Unit,
    onDelete: () -> Unit,
) {
    var expanded by remember { mutableStateOf(false) }

    Card(
        onClick = { expanded = !expanded },
        colors = CardDefaults.cardColors(
            containerColor = if (item.equipped) Crypt.copy(alpha = 0.95f) else Crypt,
        ),
        modifier = Modifier.fillMaxWidth(),
    ) {
        Column(Modifier.padding(horizontal = 12.dp, vertical = 10.dp)) {

            // ── Fila principal ────────────────────────────────────────────
            Row(verticalAlignment = Alignment.CenterVertically) {
                Text(
                    item.category.emoji(),
                    style = MaterialTheme.typography.titleMedium,
                    modifier = Modifier.padding(end = 8.dp),
                )
                Column(Modifier.weight(1f)) {
                    Text(
                        item.name,
                        style = MaterialTheme.typography.titleSmall,
                        maxLines = 1,
                        overflow = TextOverflow.Ellipsis,
                    )
                    if (item.description.isNotBlank()) {
                        Text(
                            item.description,
                            style = MaterialTheme.typography.bodySmall,
                            color = Ash,
                            maxLines = 1,
                            overflow = TextOverflow.Ellipsis,
                        )
                    }
                }
                // Cantidad
                QuantityControl(
                    quantity = item.quantity,
                    onIncrement = onIncrement,
                    onDecrement = onDecrement,
                )
            }

            // ── Fila expandida ────────────────────────────────────────────
            AnimatedVisibility(visible = expanded) {
                Column(Modifier.padding(top = 8.dp)) {
                    HorizontalDivider()
                    Spacer(Modifier.height(8.dp))

                    if (item.notes.isNotBlank()) {
                        Text(
                            item.notes,
                            style = MaterialTheme.typography.bodySmall,
                            color = Ash,
                        )
                        Spacer(Modifier.height(6.dp))
                    }

                    item.weight?.let { w ->
                        Text(
                            "Peso: %.1f lb (total: %.1f lb)".format(w, w * item.quantity),
                            style = MaterialTheme.typography.labelSmall,
                            color = Ash,
                        )
                        Spacer(Modifier.height(6.dp))
                    }

                    Row(
                        horizontalArrangement = Arrangement.spacedBy(8.dp),
                    ) {
                        // Equipar/desequipar (solo armas y armaduras)
                        if (item.category == ItemCategory.Weapon || item.category == ItemCategory.Armour) {
                            AssistChip(
                                onClick = onToggleEquipped,
                                label = {
                                    Text(if (item.equipped) "Equipado ✓" else "Equipar")
                                },
                                colors = AssistChipDefaults.assistChipColors(
                                    containerColor = if (item.equipped) Aurum.copy(alpha = 0.2f)
                                    else MaterialTheme.colorScheme.surface,
                                ),
                            )
                        }
                        // Eliminar
                        AssistChip(
                            onClick = onDelete,
                            label = { Text("Eliminar") },
                            leadingIcon = {
                                Icon(
                                    Icons.Filled.Delete,
                                    contentDescription = null,
                                    modifier = Modifier.size(16.dp),
                                    tint = Ember,
                                )
                            },
                        )
                    }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Control de cantidad +/-
// ---------------------------------------------------------------------------

@Composable
private fun QuantityControl(
    quantity: Int,
    onIncrement: () -> Unit,
    onDecrement: () -> Unit,
) {
    Row(
        verticalAlignment = Alignment.CenterVertically,
        horizontalArrangement = Arrangement.spacedBy(2.dp),
    ) {
        IconButton(onClick = onDecrement, modifier = Modifier.size(32.dp)) {
            Icon(
                Icons.Filled.Remove,
                contentDescription = "Reducir",
                modifier = Modifier.size(16.dp),
            )
        }
        Text(
            quantity.toString(),
            style = MaterialTheme.typography.titleSmall,
            modifier = Modifier.widthIn(min = 24.dp),
            textAlign = androidx.compose.ui.text.style.TextAlign.Center,
        )
        IconButton(onClick = onIncrement, modifier = Modifier.size(32.dp)) {
            Icon(
                Icons.Filled.Add,
                contentDescription = "Aumentar",
                modifier = Modifier.size(16.dp),
            )
        }
    }
}

// ---------------------------------------------------------------------------
// Diálogo añadir objeto
// ---------------------------------------------------------------------------

@Composable
private fun AddItemDialog(
    isSaving: Boolean,
    onDismiss: () -> Unit,
    onConfirm: (String, ItemCategory, String, Int, Float?, String) -> Unit,
) {
    var name by remember { mutableStateOf("") }
    var description by remember { mutableStateOf("") }
    var notes by remember { mutableStateOf("") }
    var quantityText by remember { mutableStateOf("1") }
    var weightText by remember { mutableStateOf("") }
    var selectedCategory by remember { mutableStateOf(ItemCategory.Misc) }
    var nameError by remember { mutableStateOf(false) }

    AlertDialog(
        onDismissRequest = onDismiss,
        title = { Text("Añadir objeto") },
        text = {
            Column(verticalArrangement = Arrangement.spacedBy(10.dp)) {

                // Nombre
                OutlinedTextField(
                    value = name,
                    onValueChange = { name = it; nameError = false },
                    label = { Text("Nombre *") },
                    isError = nameError,
                    singleLine = true,
                    modifier = Modifier.fillMaxWidth(),
                )

                // Categoría
                Text("Categoría", style = MaterialTheme.typography.labelMedium)
                LazyRow(horizontalArrangement = Arrangement.spacedBy(6.dp)) {
                    items(ItemCategory.entries) { cat ->
                        FilterChip(
                            selected = selectedCategory == cat,
                            onClick = { selectedCategory = cat },
                            label = { Text("${cat.emoji()} ${cat.label()}") },
                        )
                    }
                }

                // Descripción
                OutlinedTextField(
                    value = description,
                    onValueChange = { description = it },
                    label = { Text("Descripción") },
                    singleLine = true,
                    modifier = Modifier.fillMaxWidth(),
                )

                // Cantidad y peso en la misma fila
                Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
                    OutlinedTextField(
                        value = quantityText,
                        onValueChange = { quantityText = it.filter(Char::isDigit) },
                        label = { Text("Cantidad") },
                        keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Number),
                        modifier = Modifier.weight(1f),
                        singleLine = true,
                    )
                    OutlinedTextField(
                        value = weightText,
                        onValueChange = { weightText = it },
                        label = { Text("Peso (lb)") },
                        keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Decimal),
                        modifier = Modifier.weight(1f),
                        singleLine = true,
                    )
                }

                // Notas
                OutlinedTextField(
                    value = notes,
                    onValueChange = { notes = it },
                    label = { Text("Notas") },
                    modifier = Modifier.fillMaxWidth(),
                    maxLines = 2,
                )
            }
        },
        confirmButton = {
            TextButton(
                onClick = {
                    if (name.isBlank()) {
                        nameError = true; return@TextButton
                    }
                    val qty = quantityText.toIntOrNull()?.coerceAtLeast(1) ?: 1
                    val weight = weightText.toFloatOrNull()
                    onConfirm(name, selectedCategory, description, qty, weight, notes)
                },
                enabled = !isSaving,
            ) {
                if (isSaving) CircularProgressIndicator(Modifier.size(16.dp), strokeWidth = 2.dp)
                else Text("Añadir")
            }
        },
        dismissButton = {
            TextButton(onClick = onDismiss, enabled = !isSaving) { Text("Cancelar") }
        },
    )
}

// ---------------------------------------------------------------------------
// Diálogo de monedas
// ---------------------------------------------------------------------------

@Composable
private fun CurrencyDialog(
    current: Currency,
    isSaving: Boolean,
    onDismiss: () -> Unit,
    onConfirm: (Int, Int, Int, Int, Int) -> Unit,
) {
    var copper by remember { mutableStateOf(current.copper.toString()) }
    var silver by remember { mutableStateOf(current.silver.toString()) }
    var electrum by remember { mutableStateOf(current.electrum.toString()) }
    var gold by remember { mutableStateOf(current.gold.toString()) }
    var platinum by remember { mutableStateOf(current.platinum.toString()) }

    AlertDialog(
        onDismissRequest = onDismiss,
        title = { Text("Monedero") },
        text = {
            Column(verticalArrangement = Arrangement.spacedBy(10.dp)) {
                listOf(
                    Triple("Platino (PP)", platinum) { v: String -> platinum = v },
                    Triple("Oro (PO)", gold) { v: String -> gold = v },
                    Triple("Electrum (PE)", electrum) { v: String -> electrum = v },
                    Triple("Plata (PA)", silver) { v: String -> silver = v },
                    Triple("Cobre (PC)", copper) { v: String -> copper = v },
                ).forEach { (label, value, onValueChange) ->
                    OutlinedTextField(
                        value = value,
                        onValueChange = { onValueChange(it.filter(Char::isDigit)) },
                        label = { Text(label) },
                        keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Number),
                        singleLine = true,
                        modifier = Modifier.fillMaxWidth(),
                    )
                }
            }
        },
        confirmButton = {
            TextButton(
                onClick = {
                    onConfirm(
                        copper.toIntOrNull() ?: 0,
                        silver.toIntOrNull() ?: 0,
                        electrum.toIntOrNull() ?: 0,
                        gold.toIntOrNull() ?: 0,
                        platinum.toIntOrNull() ?: 0,
                    )
                },
                enabled = !isSaving,
            ) {
                if (isSaving) CircularProgressIndicator(Modifier.size(16.dp), strokeWidth = 2.dp)
                else Text("Guardar")
            }
        },
        dismissButton = {
            TextButton(onClick = onDismiss, enabled = !isSaving) { Text("Cancelar") }
        },
    )
}
