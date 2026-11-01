// Sliver Queen Avatar
// status: planned
// requires: card type Vanguard
// requires: layout Vanguard
// requires: token recipe CopyAtRandom
// Whenever you cast a non-Sliver creature spell, exile that spell. If you do, create a token that's a copy of a random non-Shapeshifter Sliver creature card with the same mana value as that spell.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && (!(q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Sliver))) && q.chars(id).is(CardType::Creature)) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_3: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; item.subject.object().into_iter().collect::<Vec<ObjectId>>() }; let moved_0 = g.exile_all(&objects_0);
let paid_0 = moved_0.clone().iter().any(|&o| g.obj(o).zone.is(ZoneKind::Exile));
g.end_sentence();
if { let q: &dyn Query = g; ((paid_0 as i64) != 0) } {
let created_2: Vec<ObjectId> = planned_value!("token recipe CopyAtRandom", Vec::<ObjectId>::new() => cantrip_mechanics::tokens::create(g, asker, { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }, &[({ let q: &dyn Query = g; (1i64).max(0) as u32 }, TokenRecipe::CopyAtRandom(CardsInOracle::And(CardsInOracle::IsNonCreatureType(CreatureType::Shapeshifter), CardsInOracle::IsCreatureType(CreatureType::Sliver), CardsInOracle::IsCardtype(CardType::Creature), CardsInOracle::ManaValueIs("Comparison(Equal, ObjectProperty { object: EventSubject, property: ManaValue, information: CurrentOrLastKnown })"))))], Arrival { tapped: false, counters: &[], states: &[] })?);
g.end_sentence();
hoisted_3.extend(created_2.clone());
}
g.end_sentence();
 Ok(()) }
planned_item!("layout-vanguard" => variant: Some(Variant::Vanguard { hand: 0, life: 7 }));

pub static CARD: CardDef = CardDef { name: "Sliver Queen Avatar", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[planned_value!("card type Vanguard", CardType::Land => CardType::Vanguard)]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::SPELL_CAST.0), ability_0_event), ability_0, "Whenever you cast a non-Sliver creature spell, exile that spell. If you do, create a token that's a copy of a random non-Shapeshifter Sliver creature card with the same mana value as that spell.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
