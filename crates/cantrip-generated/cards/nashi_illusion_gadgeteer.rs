// Nashi, Illusion Gadgeteer
// status: planned
// requires: engine: no perpetual effects
// requires: operation ChooseValueWith
// requires: operation Conjure
// Flash
// Menace, Ward {2}
// When Nashi enters, secretly choose a card in your graveyard and conjure a duplicate of it into your hand. If the duplicate isn't a land card, it perpetually gains flash.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_3_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("operation-choose-value-with" => cantrip_mechanics::choose::value(g, { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }, asker.source, ChoiceKind::Object, ChoiceTerms { options: &[|q: &dyn Query, asker: &Asker, id: ObjectId| (q.state().obj(id).zone.kind() == ZoneKind::Graveyard && Some(asker.controller) == Some(q.state().obj(id).owner))], except: &[], count: None, secret: true, unchosen: false, random: false, ordered: false, up_to: false })?);
g.end_sentence();
let duplicated_1 = { let q: &dyn Query = g; (planned_value!("depends on an instruction with a gap", Vec::<ObjectId>::new() => result_0)).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>() };
let count_1: u32 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let mut conjured_1: Vec<ObjectId> = Vec::new(); for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { conjured_1.extend(planned_value!("operation Conjure", Vec::<ObjectId>::new() => cantrip_mechanics::conjure::conjure(g, player, player, asker.source, Conjured::DuplicateOf(&duplicated_1), count_1, ConjureTo::Hand)?)); }
g.end_sentence();
if { let q: &dyn Query = g; (conjured_1.clone()).get(0 as usize).copied().is_some_and(|candidate: ObjectId| !(q.chars(candidate).is(CardType::Land))) } {
planned!("engine-no-perpetual-effects" => let objects_3: Vec<ObjectId> = { let q: &dyn Query = g; (conjured_1.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>() };
let kind_3_0 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::Effect(EffectKind::AddKeywords(Keywords::of(&[Keyword::Flash]))) };
for &object in &objects_3 { cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_3_0); });
g.end_sentence();
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Nashi, Illusion Gadgeteer", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 1, 1, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Rat,Subtype::Rogue]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flash,Keyword::Menace]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::triggers::ward::<2>("Ward {2}"),triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_3_event), ability_3, "When Nashi enters, secretly choose a card in your graveyard and conjure a duplicate of it into your hand. If the duplicate isn't a land card, it perpetually gains flash.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
