package io.github.gasparkral.dnd.ui.screen

import androidx.compose.animation.*
import androidx.compose.animation.core.animateFloatAsState
import androidx.compose.animation.core.tween
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.LazyRow
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material.icons.filled.Add
import androidx.compose.material.icons.filled.Delete
import androidx.compose.material.icons.filled.KeyboardArrowDown
import androidx.compose.material.icons.filled.Remove
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.draw.rotate
import androidx.compose.ui.graphics.Brush
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import io.github.gasparkral.dnd.model.BonusType
import io.github.gasparkral.dnd.model.Currency
import io.github.gasparkral.dnd.model.InventoryItem
import io.github.gasparkral.dnd.model.ItemCategory
import io.github.gasparkral.dnd.model.StatBonus
import io.github.gasparkral.dnd.model.formatModifier
import io.github.gasparkral.dnd.ui.component.DndDivider
import io.github.gasparkral.dnd.ui.theme.Ash
import io.github.gasparkral.dnd.ui.theme.Aurum
import io.github.gasparkral.dnd.ui.theme.Ember
import io.github.gasparkral.dnd.ui.viewmodel.InventoryViewModel
import org.koin.androidx.compose.koinViewModel
import org.koin.core.parameter.parametersOf

// ---------------------------------------------------------------------------
// Colores específicos de moneda
// ---------------------------------------------------------------------------
private val CopperColor = Color(0xFFCD7F32)
private val SilverColor = Color(0xFFB0B7BC)
private val ElectrumColor = Color(0xFF22D3EE)
private val GoldColor = Color(0xFFFBBF24)
private val PlatinumColor = Color(0xFFC084FC)

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

    var activeFilter by remember { mutableStateOf<ItemCategory?>(null) }
    val visibleItems = remember(state.items, activeFilter) {
        if (activeFilter == null) state.items
        else state.items.filter { it.category == activeFilter }
    }

    // Fondo degradado tipo dungeon
    Box(
        modifier
            .fillMaxSize()
            .background(
                Brush.verticalGradient(listOf(Color(0xFF0C0A09), Color(0xFF1C1411)))
            )
    ) {
        Column(Modifier.fillMaxSize()) {

            // ── Cabecera de mochila ───────────────────────────────────────
            BagHeader(
                itemCount = state.items.size,
                totalWeight = state.totalWeight,
                onBack = onBack,
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
                    Text("⚠ ${state.error}", color = Ember, style = MaterialTheme.typography.bodyMedium)
                }

                else -> {
                    LazyColumn(
                        contentPadding = PaddingValues(horizontal = 16.dp, vertical = 10.dp),
                        verticalArrangement = Arrangement.spacedBy(0.dp),
                    ) {

                        // ── Bolsa de monedas ──────────────────────────────
                        item {
                            Spacer(Modifier.height(12.dp))
                            CoinPouch(
                                currency = state.currency,
                                onEditClick = vm::openCurrencyDialog,
                            )
                            Spacer(Modifier.height(16.dp))
                        }

                        // ── Filtros de categoría ──────────────────────────
                        item {
                            CategoryTabs(
                                active = activeFilter,
                                onSelect = { activeFilter = if (activeFilter == it) null else it },
                            )
                            Spacer(Modifier.height(12.dp))
                        }

                        // ── Separador tipo pergamino ──────────────────────
                        item {
                            DndDivider(
                                symbol = if (visibleItems.isEmpty()) "🎒" else "⚔",
                                modifier = Modifier.padding(horizontal = 8.dp),
                            )
                            Spacer(Modifier.height(10.dp))
                        }

                        // ── Lista vacía ───────────────────────────────────
                        if (visibleItems.isEmpty()) {
                            item {
                                Box(
                                    Modifier
                                        .fillMaxWidth()
                                        .padding(vertical = 32.dp),
                                    contentAlignment = Alignment.Center,
                                ) {
                                    Column(horizontalAlignment = Alignment.CenterHorizontally) {
                                        Text("🎒", fontSize = 48.sp)
                                        Spacer(Modifier.height(8.dp))
                                        Text(
                                            if (activeFilter == null) "La mochila está vacía"
                                            else "Nada de tipo ${activeFilter!!.label()} aquí",
                                            style = MaterialTheme.typography.bodyMedium,
                                            color = Ash,
                                            textAlign = TextAlign.Center,
                                        )
                                    }
                                }
                            }
                        }

                        // ── Objetos agrupados con slot visual ─────────────
                        items(visibleItems, key = { it.id }) { item ->
                            BagSlot(
                                item = item,
                                onToggleEquipped = { vm.toggleEquipped(item) },
                                onIncrement = { vm.updateQuantity(item, item.quantity + 1) },
                                onDecrement = { vm.updateQuantity(item, item.quantity - 1) },
                                onDelete = { vm.deleteItem(item) },
                            )
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
                containerColor = Color(0xFF92400E),
                contentColor = Color(0xFFFEF3C7),
                shape = RoundedCornerShape(16.dp),
            ) {
                Row(
                    verticalAlignment = Alignment.CenterVertically,
                    modifier = Modifier.padding(horizontal = 16.dp),
                ) {
                    Icon(Icons.Filled.Add, contentDescription = null, modifier = Modifier.size(20.dp))
                    Spacer(Modifier.width(6.dp))
                    Text("Añadir", style = MaterialTheme.typography.labelLarge)
                }
            }
        }
    }

    // ── Diálogos ─────────────────────────────────────────────────────────────
    if (state.showAddDialog) {
        AddItemDialog(
            isSaving = state.isSaving,
            onDismiss = vm::closeAddDialog,
            onConfirm = { name, cat, desc, qty, weight, accessoryType, notes ->
                vm.addItem(name, cat, desc, qty, weight, accessoryType, notes)
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
// Cabecera tipo mochila de aventurero
// ---------------------------------------------------------------------------

@Composable
private fun BagHeader(
    itemCount: Int,
    totalWeight: Float,
    onBack: () -> Unit,
) {
    Box(
        Modifier
            .fillMaxWidth()
            .background(
                Brush.verticalGradient(listOf(Color(0xFF1C1208), Color(0xFF0C0A09)))
            )
    ) {
        // Línea dorada en la parte superior
        Box(
            Modifier
                .fillMaxWidth()
                .height(2.dp)
                .background(
                    Brush.horizontalGradient(
                        listOf(Color(0xFF78350F), Color(0xFFF59E0B), Color(0xFF78350F))
                    )
                )
        )

        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(horizontal = 4.dp, vertical = 12.dp)
                .padding(top = 2.dp),
            verticalAlignment = Alignment.CenterVertically,
        ) {
            IconButton(onClick = onBack) {
                Icon(Icons.AutoMirrored.Filled.ArrowBack, contentDescription = "Volver", tint = Aurum)
            }

            // Ícono de mochila
            Box(
                modifier = Modifier
                    .size(40.dp)
                    .clip(RoundedCornerShape(10.dp))
                    .background(Color(0xFF1A1208))
                    .border(1.dp, Color(0xFF78350F), RoundedCornerShape(10.dp)),
                contentAlignment = Alignment.Center,
            ) {
                Text("🎒", fontSize = 20.sp)
            }

            Spacer(Modifier.width(12.dp))

            Column(Modifier.weight(1f)) {
                Text(
                    "Mochila",
                    style = MaterialTheme.typography.titleLarge,
                    color = Color(0xFFFEF3C7),
                    fontWeight = FontWeight.Bold,
                )
                Text(
                    "$itemCount objetos · ${"%.1f".format(totalWeight)} lb",
                    style = MaterialTheme.typography.bodySmall,
                    color = Ash,
                )
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Bolsa de monedas
// ---------------------------------------------------------------------------

@Composable
private fun CoinPouch(
    currency: Currency,
    onEditClick: () -> Unit,
) {
    Column(
        modifier = Modifier
            .fillMaxWidth()
            .clip(RoundedCornerShape(16.dp))
            .background(Color(0xFF1C1208))
            .border(1.dp, Color(0xFF78350F), RoundedCornerShape(16.dp))
            .padding(16.dp),
    ) {
        Row(
            modifier = Modifier.fillMaxWidth(),
            verticalAlignment = Alignment.CenterVertically,
            horizontalArrangement = Arrangement.SpaceBetween,
        ) {
            Row(verticalAlignment = Alignment.CenterVertically) {
                Text("💰", fontSize = 18.sp)
                Spacer(Modifier.width(8.dp))
                Text(
                    "Monedero",
                    style = MaterialTheme.typography.titleSmall,
                    color = GoldColor,
                    fontWeight = FontWeight.Bold,
                )
            }
            TextButton(onClick = onEditClick) {
                Text("Editar", color = Aurum, style = MaterialTheme.typography.labelSmall)
            }
        }

        Spacer(Modifier.height(12.dp))

        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.SpaceEvenly,
        ) {
            CoinPile(amount = currency.platinum, label = "Pt", color = PlatinumColor)
            CoinPile(amount = currency.gold, label = "PO", color = GoldColor)
            CoinPile(amount = currency.electrum, label = "PE", color = ElectrumColor)
            CoinPile(amount = currency.silver, label = "PA", color = SilverColor)
            CoinPile(amount = currency.copper, label = "PC", color = CopperColor)
        }
    }
}

@Composable
private fun CoinPile(amount: Int, label: String, color: Color) {
    val active = amount > 0
    Column(
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.spacedBy(4.dp),
    ) {
        // Círculo de moneda
        Box(
            modifier = Modifier
                .size(44.dp)
                .clip(CircleShape)
                .background(if (active) color.copy(alpha = 0.15f) else Color(0xFF1C1917))
                .border(
                    width = if (active) 2.dp else 1.dp,
                    color = if (active) color else Color(0xFF292524),
                    shape = CircleShape,
                ),
            contentAlignment = Alignment.Center,
        ) {
            Text(
                text = if (amount > 999) "${amount / 1000}k" else amount.toString(),
                style = MaterialTheme.typography.labelLarge,
                fontWeight = FontWeight.ExtraBold,
                color = if (active) color else Color(0xFF44403C),
                fontSize = if (amount > 99) 11.sp else 14.sp,
            )
        }
        Text(
            label,
            style = MaterialTheme.typography.labelSmall,
            color = if (active) color.copy(alpha = 0.7f) else Color(0xFF44403C),
        )
    }
}

// ---------------------------------------------------------------------------
// Tabs de categoría (estilo scroll horizontal)
// ---------------------------------------------------------------------------

@Composable
private fun CategoryTabs(
    active: ItemCategory?,
    onSelect: (ItemCategory) -> Unit,
) {
    LazyRow(
        contentPadding = PaddingValues(horizontal = 16.dp),
        horizontalArrangement = Arrangement.spacedBy(8.dp),
    ) {
        items(ItemCategory.entries) { cat ->
            val selected = active == cat
            Box(
                modifier = Modifier
                    .clip(RoundedCornerShape(20.dp))
                    .background(
                        if (selected) Color(0xFF92400E) else Color(0xFF1C1917)
                    )
                    .border(
                        1.dp,
                        if (selected) Color(0xFFB45309) else Color(0xFF292524),
                        RoundedCornerShape(20.dp),
                    )
                    .then(Modifier.height(32.dp))
            ) {
                TextButton(
                    onClick = { onSelect(cat) },
                    modifier = Modifier.height(32.dp),
                    contentPadding = PaddingValues(horizontal = 12.dp, vertical = 0.dp),
                ) {
                    Text(
                        "${cat.emoji()} ${cat.label()}",
                        style = MaterialTheme.typography.labelMedium,
                        color = if (selected) Color(0xFFFEF3C7) else Ash,
                    )
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Slot de objeto (fila expandible con diseño de ranura de inventario)
// ---------------------------------------------------------------------------

@Composable
private fun BagSlot(
    item: InventoryItem,
    onToggleEquipped: () -> Unit,
    onIncrement: () -> Unit,
    onDecrement: () -> Unit,
    onDelete: () -> Unit,
) {
    var expanded by remember { mutableStateOf(false) }
    val arrowRotation by animateFloatAsState(
        targetValue = if (expanded) 180f else 0f,
        animationSpec = tween(200),
        label = "arrow",
    )
    val slotBg = if (item.equipped) Color(0xFF1A1208) else Color(0xFF141210)
    val slotBorder = if (item.equipped) Color(0xFF92400E) else Color(0xFF252220)

    Column(
        modifier = Modifier
            .fillMaxWidth()
            .padding(vertical = 3.dp)
            .clip(RoundedCornerShape(12.dp))
            .background(slotBg)
            .border(1.dp, slotBorder, RoundedCornerShape(12.dp)),
    ) {
        // ── Fila principal del slot ───────────────────────────────────────
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(horizontal = 12.dp, vertical = 10.dp),
            verticalAlignment = Alignment.CenterVertically,
        ) {
            // Icono de categoría en caja
            Box(
                modifier = Modifier
                    .size(38.dp)
                    .clip(RoundedCornerShape(8.dp))
                    .background(Color(0xFF0C0A09))
                    .border(1.dp, Color(0xFF292524), RoundedCornerShape(8.dp)),
                contentAlignment = Alignment.Center,
            ) {
                Text(item.category.emoji(), fontSize = 18.sp)
            }

            Spacer(Modifier.width(10.dp))

            Column(Modifier.weight(1f)) {
                Row(verticalAlignment = Alignment.CenterVertically) {
                    Text(
                        item.name,
                        style = MaterialTheme.typography.titleSmall,
                        color = Color(0xFFE7E5E4),
                        maxLines = 1,
                        overflow = TextOverflow.Ellipsis,
                        modifier = Modifier.weight(1f, fill = false),
                    )
                    if (item.equipped) {
                        Spacer(Modifier.width(6.dp))
                        Box(
                            modifier = Modifier
                                .clip(RoundedCornerShape(4.dp))
                                .background(Color(0xFF78350F))
                                .padding(horizontal = 5.dp, vertical = 1.dp),
                        ) {
                            Text(
                                "Equipado",
                                style = MaterialTheme.typography.labelSmall,
                                color = Color(0xFFFDE68A),
                                fontSize = 9.sp,
                            )
                        }
                    }
                }
                if (item.description.isNotBlank()) {
                    Text(
                        item.description,
                        style = MaterialTheme.typography.bodySmall,
                        color = Ash,
                        maxLines = 1,
                        overflow = TextOverflow.Ellipsis,
                        fontSize = 11.sp,
                    )
                }
            }

            Spacer(Modifier.width(8.dp))

            // Control de cantidad compacto
            Row(verticalAlignment = Alignment.CenterVertically) {
                IconButton(onClick = onDecrement, modifier = Modifier.size(28.dp)) {
                    Icon(Icons.Filled.Remove, null, modifier = Modifier.size(14.dp), tint = Ash)
                }
                Text(
                    "×${item.quantity}",
                    style = MaterialTheme.typography.labelLarge,
                    color = Color(0xFFFBBF24),
                    modifier = Modifier.widthIn(min = 28.dp),
                    textAlign = TextAlign.Center,
                    fontWeight = FontWeight.Bold,
                )
                IconButton(onClick = onIncrement, modifier = Modifier.size(28.dp)) {
                    Icon(Icons.Filled.Add, null, modifier = Modifier.size(14.dp), tint = Ash)
                }
            }

            // Botón expandir
            IconButton(
                onClick = { expanded = !expanded },
                modifier = Modifier.size(28.dp),
            ) {
                Icon(
                    Icons.Filled.KeyboardArrowDown,
                    contentDescription = if (expanded) "Contraer" else "Expandir",
                    tint = Color(0xFF57534E),
                    modifier = Modifier.rotate(arrowRotation),
                )
            }
        }

        // ── Panel expandido ───────────────────────────────────────────────
        AnimatedVisibility(
            visible = expanded,
            enter = expandVertically() + fadeIn(),
            exit = shrinkVertically() + fadeOut(),
        ) {
            Column(
                modifier = Modifier
                    .fillMaxWidth()
                    .background(Color(0xFF0C0A09))
                    .padding(horizontal = 14.dp, vertical = 10.dp),
                verticalArrangement = Arrangement.spacedBy(8.dp),
            ) {
                if (item.notes.isNotBlank()) {
                    Text(
                        "📜 ${item.notes}",
                        style = MaterialTheme.typography.bodySmall,
                        color = Color(0xFFA8A29E),
                        lineHeight = 18.sp,
                    )
                }

                item.weight?.let { w ->
                    Text(
                        "⚖ Peso: ${"%.1f".format(w)} lb · Total: ${"%.1f".format(w * item.quantity)} lb",
                        style = MaterialTheme.typography.labelSmall,
                        color = Color(0xFF78716C),
                    )
                }

                // Badge tipo de accesorio (solo si aplica)
                if (item.category == ItemCategory.Accessory && item.accessoryType != null) {
                    Row(
                        verticalAlignment = Alignment.CenterVertically,
                        horizontalArrangement = Arrangement.spacedBy(4.dp),
                    ) {
                        Box(
                            modifier = Modifier
                                .clip(RoundedCornerShape(6.dp))
                                .background(Color(0xFF1E1030))
                                .border(1.dp, Color(0xFF7C3AED), RoundedCornerShape(6.dp))
                                .padding(horizontal = 7.dp, vertical = 2.dp),
                        ) {
                            Text(
                                "💍 ${item.accessoryType}",
                                style = MaterialTheme.typography.labelSmall,
                                color = Color(0xFFC084FC),
                                fontSize = 10.sp,
                            )
                        }
                    }
                    Spacer(Modifier.height(4.dp))
                }

                // Badges de bonificadores de estadística
                if (item.statBonuses.isNotEmpty()) {
                    StatBonusBadges(
                        bonuses = item.statBonuses,
                        isEquipped = item.equipped,
                    )
                    Spacer(Modifier.height(4.dp))
                }

                Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
                    if (item.category == ItemCategory.Weapon
                        || item.category == ItemCategory.Armour
                        || item.category == ItemCategory.Accessory
                    ) {
                        val equipLabel = if (item.equipped) "✓ Equipado" else "Equipar"
                        OutlinedButton(
                            onClick = onToggleEquipped,
                            colors = ButtonDefaults.outlinedButtonColors(
                                contentColor = if (item.equipped) GoldColor else Ash,
                            ),
                            border = androidx.compose.foundation.BorderStroke(
                                1.dp,
                                if (item.equipped) Color(0xFF92400E) else Color(0xFF292524),
                            ),
                            contentPadding = PaddingValues(horizontal = 12.dp, vertical = 4.dp),
                            modifier = Modifier.height(32.dp),
                        ) {
                            Text(equipLabel, style = MaterialTheme.typography.labelSmall)
                        }
                    }

                    OutlinedButton(
                        onClick = onDelete,
                        colors = ButtonDefaults.outlinedButtonColors(contentColor = Ember),
                        border = androidx.compose.foundation.BorderStroke(1.dp, Color(0xFF7F1D1D)),
                        contentPadding = PaddingValues(horizontal = 12.dp, vertical = 4.dp),
                        modifier = Modifier.height(32.dp),
                    ) {
                        Icon(Icons.Filled.Delete, null, modifier = Modifier.size(12.dp))
                        Spacer(Modifier.width(4.dp))
                        Text("Descartar", style = MaterialTheme.typography.labelSmall)
                    }
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Badges de bonificadores de estadística
// ---------------------------------------------------------------------------

/**
 * Muestra una fila horizontal de badges para cada [StatBonus] de un ítem.
 * Cuando el ítem está equipado los badges se iluminan con colores positivo/negativo.
 */
@Composable
private fun StatBonusBadges(
    bonuses: List<StatBonus>,
    isEquipped: Boolean,
) {
    // Colores según estado y signo
    val activePosColor   = Color(0xFF16A34A)  // verde equipo positivo
    val activePosBorder  = Color(0xFF4ADE80)
    val activeNegColor   = Color(0xFF9F1239)  // rojo equipo negativo (maldición)
    val activeNegBorder  = Color(0xFFF87171)
    val inactiveColor    = Color(0xFF1C1917)  // gris apagado sin equipar
    val inactiveBorder   = Color(0xFF292524)

    Column(verticalArrangement = Arrangement.spacedBy(4.dp)) {
        // Título de sección
        Text(
            text = if (isEquipped) "✨ Bonificadores activos" else "🔒 Bonificadores (equipa para activar)",
            style = MaterialTheme.typography.labelSmall,
            color = if (isEquipped) Color(0xFF86EFAC) else Color(0xFF57534E),
            fontSize = 9.sp,
        )

        LazyRow(
            horizontalArrangement = Arrangement.spacedBy(6.dp),
            contentPadding = PaddingValues(vertical = 2.dp),
        ) {
            items(bonuses) { bonus ->
                val isPositive = bonus.value >= 0
                val bgColor = when {
                    !isEquipped -> inactiveColor
                    isPositive  -> activePosColor.copy(alpha = 0.15f)
                    else        -> activeNegColor.copy(alpha = 0.15f)
                }
                val borderColor = when {
                    !isEquipped -> inactiveBorder
                    isPositive  -> activePosBorder.copy(alpha = 0.6f)
                    else        -> activeNegBorder.copy(alpha = 0.6f)
                }
                val textColor = when {
                    !isEquipped -> Color(0xFF57534E)
                    isPositive  -> Color(0xFF86EFAC)
                    else        -> Color(0xFFFCA5A5)
                }
                val typeTag = when (bonus.bonusType) {
                    BonusType.Status       -> " [Estado]"
                    BonusType.Circumstance -> " [Circ.]"
                    BonusType.Item         -> ""
                    BonusType.Untyped      -> ""
                }

                Box(
                    modifier = Modifier
                        .clip(RoundedCornerShape(6.dp))
                        .background(bgColor)
                        .border(1.dp, borderColor, RoundedCornerShape(6.dp))
                        .padding(horizontal = 7.dp, vertical = 3.dp),
                ) {
                    Text(
                        text = "${bonus.stat.emoji()} ${bonus.stat.label()} ${formatModifier(bonus.value)}$typeTag",
                        style = MaterialTheme.typography.labelSmall,
                        color = textColor,
                        fontSize = 10.sp,
                        fontWeight = if (isEquipped) FontWeight.SemiBold else FontWeight.Normal,
                    )
                }
            }
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
    onConfirm: (String, ItemCategory, String, Int, Float?, String?, String) -> Unit,
) {
    var name by remember { mutableStateOf("") }
    var description by remember { mutableStateOf("") }
    var notes by remember { mutableStateOf("") }
    var quantityText by remember { mutableStateOf("1") }
    var weightText by remember { mutableStateOf("") }
    var selectedCategory by remember { mutableStateOf(ItemCategory.Misc) }
    var accessoryType by remember { mutableStateOf("") }
    var nameError by remember { mutableStateOf(false) }

    AlertDialog(
        onDismissRequest = onDismiss,
        containerColor = Color(0xFF1C1917),
        titleContentColor = Color(0xFFFEF3C7),
        title = { Text("🎒 Añadir a la mochila") },
        text = {
            Column(verticalArrangement = Arrangement.spacedBy(10.dp)) {
                OutlinedTextField(
                    value = name,
                    onValueChange = { name = it; nameError = false },
                    label = { Text("Nombre *") },
                    isError = nameError,
                    singleLine = true,
                    modifier = Modifier.fillMaxWidth(),
                )
                Text("Categoría", style = MaterialTheme.typography.labelMedium, color = Ash)
                LazyRow(horizontalArrangement = Arrangement.spacedBy(6.dp)) {
                    items(ItemCategory.entries) { cat ->
                        FilterChip(
                            selected = selectedCategory == cat,
                            onClick = { selectedCategory = cat },
                            label = { Text("${cat.emoji()} ${cat.label()}") },
                        )
                    }
                }
                OutlinedTextField(
                    value = description,
                    onValueChange = { description = it },
                    label = { Text("Descripción") },
                    singleLine = true,
                    modifier = Modifier.fillMaxWidth(),
                )
                Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
                    OutlinedTextField(
                        value = quantityText,
                        onValueChange = { quantityText = it.filter(Char::isDigit) },
                        label = { Text("Cant.") },
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
                // Campo tipo de accesorio — solo visible si la categoría es Accessory
                androidx.compose.animation.AnimatedVisibility(
                    visible = selectedCategory == ItemCategory.Accessory,
                ) {
                    OutlinedTextField(
                        value = accessoryType,
                        onValueChange = { accessoryType = it },
                        label = { Text("💍 Tipo de accesorio") },
                        placeholder = { Text("ej: botas, anillo, capucha…") },
                        singleLine = true,
                        modifier = Modifier.fillMaxWidth(),
                    )
                }
                OutlinedTextField(
                    value = notes,
                    onValueChange = { notes = it },
                    label = { Text("Notas mágicas o especiales") },
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
                    val accType = if (selectedCategory == ItemCategory.Accessory && accessoryType.isNotBlank())
                        accessoryType else null
                    onConfirm(name, selectedCategory, description, qty, weightText.toFloatOrNull(), accType, notes)
                },
                enabled = !isSaving,
            ) {
                if (isSaving) CircularProgressIndicator(Modifier.size(16.dp), strokeWidth = 2.dp)
                else Text("Guardar", color = Aurum)
            }
        },
        dismissButton = {
            TextButton(onClick = onDismiss, enabled = !isSaving) {
                Text("Cancelar", color = Ash)
            }
        },
    )
}

// ---------------------------------------------------------------------------
// Diálogo editar monedas
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
        containerColor = Color(0xFF1C1208),
        titleContentColor = GoldColor,
        title = { Text("💰 Monedero") },
        text = {
            Column(verticalArrangement = Arrangement.spacedBy(10.dp)) {
                listOf(
                    Triple("🟣 Platino (Pt)", platinum, PlatinumColor) to { v: String -> platinum = v },
                    Triple("🟡 Oro (PO)", gold, GoldColor) to { v: String -> gold = v },
                    Triple("🔵 Electrum (PE)", electrum, ElectrumColor) to { v: String -> electrum = v },
                    Triple("⚪ Plata (PA)", silver, SilverColor) to { v: String -> silver = v },
                    Triple("🟠 Cobre (PC)", copper, CopperColor) to { v: String -> copper = v },
                ).forEach { (triple, onChange) ->
                    val (label, value, color) = triple
                    OutlinedTextField(
                        value = value,
                        onValueChange = { onChange(it.filter(Char::isDigit)) },
                        label = { Text(label, color = color) },
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
                else Text("Guardar", color = GoldColor)
            }
        },
        dismissButton = {
            TextButton(onClick = onDismiss, enabled = !isSaving) {
                Text("Cancelar", color = Ash)
            }
        },
    )
}
