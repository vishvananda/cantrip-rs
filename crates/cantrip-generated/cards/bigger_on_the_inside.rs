// Bigger on the Inside
// status: planned
// requires: operation ChangeSpell
// Enchant artifact or land
// Enchanted permanent has "{T}: Target player adds two mana of any one color. The next spell they cast this turn has cascade." (When they cast their next spell, they exile cards from the top of their library until they exile a nonland card that costs less. They may cast it without paying its mana cost. They put the exiled cards on the bottom in a random order.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_enchant(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if cantrip_mechanics::targets::permanent(q, id) && ((q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Land)))) }
#[allow(unused_variables)] fn granted_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Player(p) if true) }
static GRANTED_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(granted_0_target_0)];
fn granted_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let mana_players_2 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.player()).into_iter().collect::<Vec<PlayerId>>() };
let mana_2 = { let q: &dyn Query = g; (2i64).max(0) as u32 };
for player in mana_players_2 {
if mana_2 > 0 { let colors = cantrip_mechanics::choose_colors(g, player, asker.source, ColorSet(31), 1)?;
g.add_mana(player, colors[0].mana(), mana_2);}
}
g.end_sentence();
planned!("operation-change-spell" => cantrip_mechanics::spell_changes::next_spell(g, asker, { let q: &dyn Query = g; t_0_raw.get(0).copied().and_then(|t: Target| t.player()).expect("a reference that always names a player") }, |q: &dyn Query, asker: &Asker, id: ObjectId| true, NextSpell { this_turn: true, from_hand: false }, &[SpellChange::Gains("Mechanic")]));
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let card = { let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }; let _ = card; if let Some(slot) = chars.abilities.iter().position(|a| a.is_none()) { chars.abilities[slot] = Some(AbilityRef::delayed({ let o = q.state().obj(asker.source); o.copied_card().unwrap_or(o.card) }, 0)); } }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Bigger on the Inside", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 1, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Aura]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[static_ability(Layer::Ability, cantrip_mechanics::attach::attached_object, ability_1_apply_0, "Enchanted permanent has \"{T}: Target player adds two mana of any one color. The next spell they cast this turn has cascade.\"")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), granted_0, "{T}: Target player adds two mana of any one color. The next spell they cast this turn has cascade.").targets(GRANTED_0_TARGETS)], enters: None, modal: None, enchant: Some(target(ability_0_enchant)), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
