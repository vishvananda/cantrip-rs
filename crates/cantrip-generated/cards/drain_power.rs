// Drain Power
// status: planned
// requires: engine: no effect empties a mana pool
// requires: keyword action ActivateManaAbility
// requires: mana production LostThisWay
// Target player activates a mana ability of each land they control. Then that player loses all unspent mana and you add the mana lost this way.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Player(p) if true) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0)];
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
planned!("depends-on-an-instruction-with-a-gap" => g.begin_simultaneous());
for bound_4294967294 in { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Land) && t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.player()) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() } {
planned!("keyword-action-activate-mana-ability" => if let Some(player) = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.player()) } { if let Some(object) = ({ let q: &dyn Query = g; (Some(bound_4294967294).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }).first().copied() { cantrip_mechanics::mana::activate_mana_ability(g, player, object)?; }; });
}
planned!("engine-no-effect-empties-a-mana-pool" => if let Some(player) = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.player()) } { g.empty_mana_pool(player); });
planned!("depends-on-an-instruction-with-a-gap" => g.end_simultaneous());
g.end_sentence();
planned!("mana-production-lost-this-way" => for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { cantrip_mechanics::mana::add_production(g, player, Production::LostThisWay(0), None, None)?; });
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Drain Power", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(ABILITY_0_TARGETS, ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
