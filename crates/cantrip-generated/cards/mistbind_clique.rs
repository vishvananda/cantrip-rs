// Mistbind Clique
// status: planned
// requires: engine: no Champion
// requires: trigger event Pair ChampionedWith
// Flash
// Flying
// Champion a Faerie (When this enters, sacrifice it unless you exile another Faerie you control. When this leaves the battlefield, that card returns to the battlefield.)
// When a Faerie is championed with this creature, tap all lands target player controls.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_champion_0(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Faerie)) }
#[allow(unused_variables)] fn ability_3_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Player(p) if true) }
static ABILITY_3_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_3_target_0)];
#[allow(unused_variables)] fn ability_3_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
fn ability_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let objects_1 = { let q: &dyn Query = g; (q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Land) && t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.player()) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let turned_1: Vec<ObjectId> = objects_1.iter().copied().filter(|&o| g.state().obj(o).tapped != true).collect(); g.tap_together(&turned_1);
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }
planned_item!("engine-no-champion" => abilities: cantrip_mechanics::champion::enters(ability_2_champion_0));
planned_item!("engine-no-champion" => abilities: cantrip_mechanics::champion::LEAVES);
planned_item!("trigger-event-pair-championed-with" => matcher: #[allow(unused_variables)] fn ability_3_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Championed { card, champion } = event { let id = *card; let other = *champion; let subject = Target::Object(id); let data = [other.0, u32::MAX]; if q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Faerie)) && Some(asker.source) == Some(other) { return Some(Fired { subject, data }); } }
 None });

pub static CARD: CardDef = CardDef { name: "Mistbind Clique", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Faerie,Subtype::Wizard]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flash,Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(planned_value!("trigger event Pair ChampionedWith", EventMask(0) => EventMask(EventMask::CHAMPIONED.0)), ability_3_event), ability_3, "When a Faerie is championed with this creature, tap all lands target player controls.").targets(ABILITY_3_TARGETS)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
