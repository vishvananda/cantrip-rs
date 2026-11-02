// Amalia Benavides Aguirre
// status: ready
// Ward—Pay 3 life.
// Whenever you gain life, Amalia Benavides Aguirre explores. Then destroy all other creatures if its power is exactly 20. (To have this creature explore, reveal the top card of your library. Put that card into your hand if it's a land. Otherwise, put a +1/+1 counter on this creature, then put the card back or put it into your graveyard.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::LifeChange { player, delta } = event { let subject = Target::Player(*player); let data = [delta.unsigned_abs(), 0]; if *delta > 0 && Some(asker.controller) == Some(*player) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_3: Vec<ObjectId> = Vec::new();
let mut hoisted_2: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
if let Some(object) = ({ let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }).first().copied() { cantrip_mechanics::explore(g, object)?; }
g.end_sentence();
if { let q: &dyn Query = g; Some(asker.source).is_some_and(|candidate: ObjectId| q.chars(candidate).power.is_some_and(|v| (i64::from(v)) == (20i64))) } {
let objects_1 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (!(Some(asker.source) == Some(id)) && q.chars(id).is(CardType::Creature))).collect::<Vec<ObjectId>>() }; let destroyed_1 = g.destroy_all(&objects_1);
g.end_sentence();
hoisted_2.extend(destroyed_1.clone());
hoisted_3.extend({ let q: &dyn Query = g; objects_1.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Amalia Benavides Aguirre", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [1, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Vampire,Subtype::Scout]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::triggers::ward_life::<3>("Ward—Pay 3 life."),triggered(when_asked(EventMask(EventMask::LIFE_CHANGE.0), ability_1_event), ability_1, "Whenever you gain life, Amalia Benavides Aguirre explores. Then destroy all other creatures if its power is exactly 20.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
