// Stonehewer Giant Avatar
// status: planned
// requires: card type Vanguard
// requires: layout Vanguard
// requires: token recipe CopyAtRandom
// Whenever a creature you control enters, create a token that's a copy of a random Equipment card with mana value less than that creature's mana value. Attach that Equipment to that creature.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && (q.chars(id).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(id))) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let created_1: Vec<ObjectId> = planned_value!("token recipe CopyAtRandom", Vec::<ObjectId>::new() => cantrip_mechanics::tokens::create_from_recipes(g, asker.source, &{ let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }, &[({ let q: &dyn Query = g; (1i64).max(0) as u32 }, TokenRecipe::CopyAtRandom(CardsInOracle::And(&[CardsInOracle::IsArtifactType(ArtifactType::Equipment), CardsInOracle::ManaValueIs("Comparison(Less, ObjectProperty { object: EventSubject, property: ManaValue, information: CurrentOrLastKnown })")])))], Arrival { tapped: false, counters: &[], states: &[] })?);
g.end_sentence();
let attachments_2 = { let q: &dyn Query = g; (created_1.clone()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let host_2 = { let q: &dyn Query = g; item.subject.object() }; if let Some(host) = host_2 { cantrip_api::instruction(g, |g| { for attachment in attachments_2 { g.attach(attachment, host); } }); }
g.end_sentence();
 Ok(()) }
planned_item!("layout-vanguard" => variant: Some(Variant::Vanguard { hand: 1, life: -5 }));

pub static CARD: CardDef = CardDef { name: "Stonehewer Giant Avatar", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[planned_value!("card type Vanguard", CardType::Land => CardType::Vanguard)]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_0_event), ability_0, "Whenever a creature you control enters, create a token that's a copy of a random Equipment card with mana value less than that creature's mana value. Attach that Equipment to that creature.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
