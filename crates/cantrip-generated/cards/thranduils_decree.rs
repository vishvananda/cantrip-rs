// Thranduil's Decree
// status: ready
// Counter target spell. If a permanent spell is countered this way, exile it instead of putting it into its owner's graveyard. You may cast that card without paying its mana cost for as long as it remains exiled.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Object(id) if (q.state().obj(id).zone.kind() == ZoneKind::Stack) && (true)) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0)];
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_3: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
if { let q: &dyn Query = g; t_0_raw.get(0).copied().and_then(|t: Target| t.object()).is_some_and(|candidate: ObjectId| (q.chars(candidate).is(CardType::Artifact) || q.chars(candidate).is(CardType::Battle) || q.chars(candidate).is(CardType::Creature) || q.chars(candidate).is(CardType::Enchantment) || q.chars(candidate).is(CardType::Land) || q.chars(candidate).is(CardType::Planeswalker))) } {
let objects_0 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() }; cantrip_api::instruction(g, |g| { for spell in objects_0 { g.counter_to(spell, CounterTo::Exile); } });
let countered_1: Vec<ObjectId> = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() }.into_iter().filter_map(|o: ObjectId| g.state().obj(o).became).collect();
g.end_sentence();
let cards_2_0 = { let q: &dyn Query = g; (countered_1.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>() }; let players_2_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; cantrip_api::instruction(g, |g| { for &card in &cards_2_0 { if g.obj(card).zone.is(ZoneKind::Exile) { for &player in &players_2_0 { g.allow_play(card, player, PlayGrant { lands: false, expiry: Expiry::Never, instead: Some(ManaCost::ZERO), ..PlayGrant::CAST }); } } } });
g.end_sentence();
hoisted_3.extend(countered_1.clone());
} else {
let objects_4 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.object()).into_iter().collect::<Vec<_>>() }; cantrip_api::instruction(g, |g| { for spell in objects_4 { g.counter(spell); } });
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Thranduil's Decree", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(ABILITY_0_TARGETS, ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
