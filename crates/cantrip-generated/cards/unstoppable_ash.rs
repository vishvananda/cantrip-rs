// Unstoppable Ash
// status: planned
// requires: engine: no Champion
// Trample
// Champion a Treefolk or Warrior (When this enters, sacrifice it unless you exile another Treefolk or Warrior you control. When this leaves the battlefield, that card returns to the battlefield.)
// Whenever a creature you control becomes blocked, it gets +0/+5 until end of turn.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_champion_0(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { (q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Treefolk)) || q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Warrior))) }
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Blocks { attacker, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::MAX, id.0]; if (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id))) { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_1 = { let q: &dyn Query = g; (item.subject.object().into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_1 = { let q: &dyn Query = g; Some(asker.source) }; let duration_1 = Duration::EndOfTurn;
let kind_1_0_0 = { let q: &dyn Query = g; EffectKind::Pump((0i64) as i16, (5i64) as i16) };
if let Some(source) = source_1 { cantrip_api::instruction(g, |g| { for &object in &objects_1 { g.add_effect(source, object, kind_1_0_0, duration_1);  } }); }
g.end_sentence();
 Ok(()) }
planned_item!("engine-no-champion" => abilities: cantrip_mechanics::champion::enters(ability_1_champion_0));
planned_item!("engine-no-champion" => abilities: cantrip_mechanics::champion::LEAVES);

pub static CARD: CardDef = CardDef { name: "Unstoppable Ash", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Treefolk,Subtype::Warrior]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(5), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Trample]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::BLOCKS.0), ability_2_event), ability_2, "Whenever a creature you control becomes blocked, it gets +0/+5 until end of turn.").targets(&[]).once_per_batch().batch_by_data()], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
