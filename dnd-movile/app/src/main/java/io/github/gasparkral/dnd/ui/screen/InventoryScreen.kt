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
import androidx.compose.material.icons.filled.Close
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
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import androidx.compose.foundation.layout.imePadding
import io.github.gasparkral.dnd.model.AddItemRequest
import io.github.gasparkral.dnd.model.BonusStat
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
            onConfirm = { request -> vm.addItem(request) },
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
// Diálogo añadir objeto — dinámico por categoría
// ---------------------------------------------------------------------------

/** Datos extra específicos por categoría. Se resetean al cambiar de categoría. */
private data class CategoryExtras(
    // Weapon
    val damageDice: String = "",
    val damageType: String = "slashing",
    val weaponKind: String = "simple",
    val rangeNormal: String = "",
    val rangeLong: String = "",
    val properties: String = "",
    // Armour
    val armourCategory: String = "light",
    val baseAc: String = "",
    val dexCap: String = "",
    val strReq: String = "",
    val stealthDisadv: Boolean = false,
    // Consumable
    val consumableSubtype: String = "potion",
    val effect: String = "",
    val duration: String = "",
    // Accessory / Tool
    val accessorySubtype: String = "",
    val statBonuses: List<DraftBonus> = emptyList(),
    // Treasure
    val gpValue: String = "",
    val treasureType: String = "gem",
)

private data class DraftBonus(
    val stat: BonusStat = BonusStat.Strength,
    val value: String = "0",
    val bonusType: BonusType = BonusType.Item,
)

@Composable
private fun AddItemDialog(
    isSaving: Boolean,
    onDismiss: () -> Unit,
    onConfirm: (AddItemRequest) -> Unit,
) {
    var name            by remember { mutableStateOf("") }
    var description     by remember { mutableStateOf("") }
    var notes           by remember { mutableStateOf("") }
    var quantityText    by remember { mutableStateOf("1") }
    var weightText      by remember { mutableStateOf("") }
    var selectedCategory by remember { mutableStateOf(ItemCategory.Misc) }
    var nameError       by remember { mutableStateOf(false) }
    var extras          by remember { mutableStateOf(CategoryExtras()) }

    LaunchedEffect(selectedCategory) { extras = CategoryExtras() }

    AlertDialog(
        onDismissRequest = onDismiss,
        containerColor = Color(0xFF1C1917),
        titleContentColor = Color(0xFFFEF3C7),
        title = { Text("${selectedCategory.emoji()} Añadir a la mochila") },
        text = {
            Column(
                modifier = Modifier
                    .verticalScroll(rememberScrollState())
                    .imePadding(),
                verticalArrangement = Arrangement.spacedBy(10.dp),
            ) {
                // ── Nombre ────────────────────────────────────────────────
                OutlinedTextField(
                    value = name,
                    onValueChange = { name = it; nameError = false },
                    label = { Text("Nombre *") },
                    isError = nameError,
                    singleLine = true,
                    modifier = Modifier.fillMaxWidth(),
                )

                // ── Selector de categoría ─────────────────────────────────
                DialogLabel("Categoría")
                LazyRow(horizontalArrangement = Arrangement.spacedBy(6.dp)) {
                    items(ItemCategory.entries) { cat ->
                        FilterChip(
                            selected = selectedCategory == cat,
                            onClick = { selectedCategory = cat },
                            label = { Text("${cat.emoji()} ${cat.label()}") },
                        )
                    }
                }

                // ── Descripción ───────────────────────────────────────────
                OutlinedTextField(
                    value = description,
                    onValueChange = { description = it },
                    label = { Text("Descripción") },
                    singleLine = true,
                    modifier = Modifier.fillMaxWidth(),
                )

                // ── Campos específicos por categoría ──────────────────────
                when (selectedCategory) {
                    ItemCategory.Weapon     -> WeaponExtrasSection(extras)     { extras = it }
                    ItemCategory.Armour     -> ArmourExtrasSection(extras)     { extras = it }
                    ItemCategory.Consumable -> ConsumableExtrasSection(extras) { extras = it }
                    ItemCategory.Accessory  -> AccessoryExtrasSection(extras)  { extras = it }
                    ItemCategory.Treasure   -> TreasureExtrasSection(extras)   { extras = it }
                    ItemCategory.Tool       -> ToolExtrasSection(extras)       { extras = it }
                    ItemCategory.Misc       -> { /* solo campos base */ }
                }

                // ── Cantidad + Peso ───────────────────────────────────────
                Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
                    OutlinedTextField(
                        value = quantityText,
                        onValueChange = { quantityText = it.filter(Char::isDigit) },
                        label = { Text("Cant.") },
                        singleLine = true,
                        keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Number),
                        modifier = Modifier.weight(1f),
                    )
                    OutlinedTextField(
                        value = weightText,
                        onValueChange = { weightText = it },
                        label = { Text("Peso (lb)") },
                        singleLine = true,
                        keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Decimal),
                        modifier = Modifier.weight(1f),
                    )
                }

                // ── Notas ─────────────────────────────────────────────────
                OutlinedTextField(
                    value = notes,
                    onValueChange = { notes = it },
                    label = { Text("Notas") },
                    singleLine = true,
                    modifier = Modifier.fillMaxWidth(),
                )
            }
        },
        confirmButton = {
            TextButton(
                enabled = !isSaving,
                onClick = {
                    if (name.isBlank()) { nameError = true; return@TextButton }
                    val qty    = quantityText.toIntOrNull()?.coerceAtLeast(1) ?: 1
                    val weight = weightText.toFloatOrNull()

                    val statBonuses = when (selectedCategory) {
                        ItemCategory.Accessory -> extras.statBonuses.mapNotNull { draft ->
                            val v = draft.value.toIntOrNull() ?: return@mapNotNull null
                            StatBonus(draft.stat, v, draft.bonusType)
                        }
                        else -> emptyList()
                    }
                    val accType = when (selectedCategory) {
                        ItemCategory.Accessory -> extras.accessorySubtype.ifBlank { null }
                        else -> null
                    }

                    onConfirm(
                        AddItemRequest(
                            name          = name.trim(),
                            category      = selectedCategory,
                            description   = description.trim(),
                            quantity      = qty,
                            weight        = weight,
                            accessoryType = accType,
                            statBonuses   = statBonuses,
                            notes         = buildEnrichedNotes(selectedCategory, extras, notes),
                        )
                    )
                },
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
// Secciones específicas por categoría
// ---------------------------------------------------------------------------

@Composable
private fun WeaponExtrasSection(extras: CategoryExtras, onChange: (CategoryExtras) -> Unit) {
    val damageTypes = listOf(
        "slashing", "piercing", "bludgeoning", "fire", "cold",
        "lightning", "thunder", "poison", "acid", "psychic", "radiant", "necrotic", "force"
    )
    DialogLabel("⚔️ Datos de arma")
    Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
        OutlinedTextField(
            value = extras.damageDice,
            onValueChange = { onChange(extras.copy(damageDice = it)) },
            label = { Text("Daño (ej. 1d8)") },
            singleLine = true,
            modifier = Modifier.weight(1f),
        )
        DialogDropdown(
            label = "Tipo daño",
            options = damageTypes,
            selected = extras.damageType,
            onSelect = { onChange(extras.copy(damageType = it)) },
            modifier = Modifier.weight(1f),
        )
    }
    Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
        DialogDropdown(
            label = "Categoría",
            options = listOf("simple", "martial"),
            labels  = listOf("Simple", "Marcial"),
            selected = extras.weaponKind,
            onSelect = { onChange(extras.copy(weaponKind = it)) },
            modifier = Modifier.weight(1f),
        )
        OutlinedTextField(
            value = extras.properties,
            onValueChange = { onChange(extras.copy(properties = it)) },
            label = { Text("Propiedades") },
            placeholder = { Text("finesse, light…") },
            singleLine = true,
            modifier = Modifier.weight(1f),
        )
    }
    Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
        OutlinedTextField(
            value = extras.rangeNormal,
            onValueChange = { onChange(extras.copy(rangeNormal = it)) },
            label = { Text("Alcance normal") },
            singleLine = true,
            keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Number),
            modifier = Modifier.weight(1f),
        )
        OutlinedTextField(
            value = extras.rangeLong,
            onValueChange = { onChange(extras.copy(rangeLong = it)) },
            label = { Text("Alcance largo") },
            singleLine = true,
            keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Number),
            modifier = Modifier.weight(1f),
        )
    }
}

@Composable
private fun ArmourExtrasSection(extras: CategoryExtras, onChange: (CategoryExtras) -> Unit) {
    DialogLabel("🛡️ Datos de armadura")
    Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
        DialogDropdown(
            label = "Categoría",
            options = listOf("light", "medium", "heavy", "shield"),
            labels  = listOf("Ligera", "Media", "Pesada", "Escudo"),
            selected = extras.armourCategory,
            onSelect = { onChange(extras.copy(armourCategory = it)) },
            modifier = Modifier.weight(1f),
        )
        OutlinedTextField(
            value = extras.baseAc,
            onValueChange = { onChange(extras.copy(baseAc = it)) },
            label = { Text("CA base") },
            singleLine = true,
            keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Number),
            modifier = Modifier.weight(1f),
        )
    }
    if (extras.armourCategory == "medium") {
        OutlinedTextField(
            value = extras.dexCap,
            onValueChange = { onChange(extras.copy(dexCap = it)) },
            label = { Text("Cap DES (máx)") },
            singleLine = true,
            keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Number),
            modifier = Modifier.fillMaxWidth(),
        )
    }
    if (extras.armourCategory == "heavy") {
        OutlinedTextField(
            value = extras.strReq,
            onValueChange = { onChange(extras.copy(strReq = it)) },
            label = { Text("Req. FUE mínima") },
            singleLine = true,
            keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Number),
            modifier = Modifier.fillMaxWidth(),
        )
    }
    Row(
        verticalAlignment = Alignment.CenterVertically,
        horizontalArrangement = Arrangement.spacedBy(4.dp),
    ) {
        Checkbox(
            checked = extras.stealthDisadv,
            onCheckedChange = { onChange(extras.copy(stealthDisadv = it)) },
        )
        Text("Desventaja en Sigilo", style = MaterialTheme.typography.bodyMedium, color = Color(0xFFA8A29E))
    }
}

@Composable
private fun ConsumableExtrasSection(extras: CategoryExtras, onChange: (CategoryExtras) -> Unit) {
    DialogLabel("🧪 Datos de consumible")
    DialogDropdown(
        label = "Subtipo",
        options = listOf("potion", "scroll", "food", "poison", "other"),
        labels  = listOf("Poción", "Pergamino", "Comida/Bebida", "Veneno", "Otro"),
        selected = extras.consumableSubtype,
        onSelect = { onChange(extras.copy(consumableSubtype = it)) },
        modifier = Modifier.fillMaxWidth(),
    )
    OutlinedTextField(
        value = extras.effect,
        onValueChange = { onChange(extras.copy(effect = it)) },
        label = { Text("Efecto (ej. Cura 2d4+2 PG)") },
        singleLine = true,
        modifier = Modifier.fillMaxWidth(),
    )
    OutlinedTextField(
        value = extras.duration,
        onValueChange = { onChange(extras.copy(duration = it)) },
        label = { Text("Duración (ej. 1 hora)") },
        singleLine = true,
        modifier = Modifier.fillMaxWidth(),
    )
}

@Composable
private fun AccessoryExtrasSection(extras: CategoryExtras, onChange: (CategoryExtras) -> Unit) {
    DialogLabel("💍 Datos de accesorio")
    DialogDropdown(
        label = "Tipo de accesorio",
        options = listOf("ring", "amulet", "cloak", "belt", "boots", "gloves", "helmet", "bracers", "other"),
        labels  = listOf("Anillo", "Amuleto", "Capa", "Cinturón", "Botas", "Guantes", "Yelmo", "Brazaletes", "Otro"),
        selected = extras.accessorySubtype.ifBlank { "ring" },
        onSelect = { onChange(extras.copy(accessorySubtype = it)) },
        modifier = Modifier.fillMaxWidth(),
    )
    DialogLabel("Bonificadores de estadística")
    extras.statBonuses.forEachIndexed { idx, draft ->
        StatBonusRow(
            draft    = draft,
            onChange = { updated ->
                val list = extras.statBonuses.toMutableList().also { it[idx] = updated }
                onChange(extras.copy(statBonuses = list))
            },
            onRemove = {
                val list = extras.statBonuses.toMutableList().also { it.removeAt(idx) }
                onChange(extras.copy(statBonuses = list))
            },
        )
    }
    TextButton(onClick = { onChange(extras.copy(statBonuses = extras.statBonuses + DraftBonus())) }) {
        Icon(Icons.Filled.Add, contentDescription = null, modifier = Modifier.size(16.dp))
        Spacer(Modifier.width(4.dp))
        Text("Añadir bonificador")
    }
}

@Composable
private fun StatBonusRow(
    draft: DraftBonus,
    onChange: (DraftBonus) -> Unit,
    onRemove: () -> Unit,
) {
    val stats      = BonusStat.entries
    val bonusTypes = BonusType.entries

    Row(
        verticalAlignment = Alignment.CenterVertically,
        horizontalArrangement = Arrangement.spacedBy(4.dp),
        modifier = Modifier.fillMaxWidth(),
    ) {
        DialogDropdown(
            label    = "Stat",
            options  = stats.map { it.name },
            labels   = stats.map { it.label() },
            selected = draft.stat.name,
            onSelect = { name -> onChange(draft.copy(stat = BonusStat.valueOf(name))) },
            modifier = Modifier.weight(2f),
        )
        OutlinedTextField(
            value = draft.value,
            onValueChange = { onChange(draft.copy(value = it)) },
            label = { Text("Val.") },
            singleLine = true,
            keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Number),
            modifier = Modifier.weight(1f),
        )
        DialogDropdown(
            label    = "Tipo",
            options  = bonusTypes.map { it.name },
            labels   = listOf("Item", "Estado", "Circ.", "Sin tipo"),
            selected = draft.bonusType.name,
            onSelect = { name -> onChange(draft.copy(bonusType = BonusType.valueOf(name))) },
            modifier = Modifier.weight(1.5f),
        )
        IconButton(onClick = onRemove, modifier = Modifier.size(36.dp)) {
            Icon(Icons.Filled.Close, contentDescription = "Eliminar", tint = Color(0xFF78716C), modifier = Modifier.size(18.dp))
        }
    }
}

@Composable
private fun TreasureExtrasSection(extras: CategoryExtras, onChange: (CategoryExtras) -> Unit) {
    DialogLabel("💎 Datos de tesoro")
    Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
        DialogDropdown(
            label    = "Tipo",
            options  = listOf("gem", "art", "jewellery", "coin", "other"),
            labels   = listOf("Gema", "Obra de arte", "Joyería", "Moneda especial", "Otro"),
            selected = extras.treasureType,
            onSelect = { onChange(extras.copy(treasureType = it)) },
            modifier = Modifier.weight(1f),
        )
        OutlinedTextField(
            value = extras.gpValue,
            onValueChange = { onChange(extras.copy(gpValue = it)) },
            label = { Text("Valor (PO)") },
            singleLine = true,
            keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Number),
            modifier = Modifier.weight(1f),
        )
    }
}

@Composable
private fun ToolExtrasSection(extras: CategoryExtras, onChange: (CategoryExtras) -> Unit) {
    DialogLabel("🔧 Herramienta")
    OutlinedTextField(
        value = extras.accessorySubtype,
        onValueChange = { onChange(extras.copy(accessorySubtype = it)) },
        label = { Text("Tipo (ej. Instrumentos de ladrón)") },
        singleLine = true,
        modifier = Modifier.fillMaxWidth(),
    )
}

// ---------------------------------------------------------------------------
// Helpers de UI compartidos por las secciones
// ---------------------------------------------------------------------------

@Composable
private fun DialogLabel(text: String) {
    Text(
        text = text,
        style = MaterialTheme.typography.labelSmall,
        color = Color(0xFF78716C),
        modifier = Modifier.padding(top = 4.dp),
    )
}

/**
 * Dropdown compacto para usar dentro de diálogos.
 * Usa ExposedDropdownMenuBox de Material3 para que los taps funcionen
 * correctamente dentro de AlertDialog en Android.
 */
@OptIn(ExperimentalMaterial3Api::class)
@Composable
private fun DialogDropdown(
    label: String,
    options: List<String>,
    labels: List<String> = options,
    selected: String,
    onSelect: (String) -> Unit,
    modifier: Modifier = Modifier,
) {
    var expanded by remember { mutableStateOf(false) }
    val selectedLabel = labels.getOrNull(options.indexOf(selected)) ?: selected

    ExposedDropdownMenuBox(
        expanded = expanded,
        onExpandedChange = { expanded = it },
        modifier = modifier,
    ) {
        OutlinedTextField(
            value = selectedLabel,
            onValueChange = {},
            label = { Text(label) },
            readOnly = true,
            trailingIcon = { ExposedDropdownMenuDefaults.TrailingIcon(expanded = expanded) },
            modifier = Modifier
                .menuAnchor()
                .fillMaxWidth(),
            singleLine = true,
        )
        ExposedDropdownMenu(
            expanded = expanded,
            onDismissRequest = { expanded = false },
        ) {
            options.forEachIndexed { i, opt ->
                DropdownMenuItem(
                    text = { Text(labels.getOrElse(i) { opt }) },
                    onClick = { onSelect(opt); expanded = false },
                )
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Enriquecer notas con campos específicos de la categoría
// ---------------------------------------------------------------------------

private fun buildEnrichedNotes(
    category: ItemCategory,
    extras: CategoryExtras,
    baseNotes: String,
): String {
    val parts = mutableListOf<String>()
    when (category) {
        ItemCategory.Weapon -> {
            if (extras.damageDice.isNotBlank())
                parts += "Daño: ${extras.damageDice} ${extras.damageType}"
            if (extras.weaponKind.isNotBlank())
                parts += "Tipo: ${extras.weaponKind}"
            if (extras.rangeNormal.isNotBlank())
                parts += "Alcance: ${extras.rangeNormal}/${extras.rangeLong.ifBlank { "?" }} ft."
            if (extras.properties.isNotBlank())
                parts += "Propiedades: ${extras.properties}"
        }
        ItemCategory.Armour -> {
            if (extras.baseAc.isNotBlank()) {
                val acStr = when (extras.armourCategory) {
                    "light"  -> "CA: ${extras.baseAc} + DES"
                    "medium" -> "CA: ${extras.baseAc} + DES (máx ${extras.dexCap.ifBlank { "2" }})"
                    "heavy"  -> "CA: ${extras.baseAc}"
                    "shield" -> "CA: +${extras.baseAc}"
                    else     -> "CA: ${extras.baseAc}"
                }
                parts += acStr
            }
            if (extras.strReq.isNotBlank()) parts += "Req. FUE: ${extras.strReq}"
            if (extras.stealthDisadv)        parts += "Desventaja en Sigilo"
        }
        ItemCategory.Consumable -> {
            if (extras.consumableSubtype.isNotBlank()) parts += "Subtipo: ${extras.consumableSubtype}"
            if (extras.effect.isNotBlank())            parts += "Efecto: ${extras.effect}"
            if (extras.duration.isNotBlank())          parts += "Duración: ${extras.duration}"
        }
        ItemCategory.Treasure -> {
            if (extras.treasureType.isNotBlank()) parts += "Tipo: ${extras.treasureType}"
            if (extras.gpValue.isNotBlank())       parts += "Valor: ${extras.gpValue} PO"
        }
        ItemCategory.Tool -> {
            if (extras.accessorySubtype.isNotBlank()) parts += "Tipo: ${extras.accessorySubtype}"
        }
        else -> {}
    }
    if (baseNotes.isNotBlank()) parts += baseNotes
    return parts.joinToString(" · ")
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
