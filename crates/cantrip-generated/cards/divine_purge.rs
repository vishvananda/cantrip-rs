// Divine Purge
// status: planned
// requires: engine: no perpetual effects
// Exile all artifacts and creatures with mana value 3 or less. They perpetually gain "This spell costs {2} more to cast" and "This permanent enters tapped." For as long as each of them remain exiled, its owner may play it.
#[allow(unused_imports)] use cantrip_api::*;
static PERPETUAL_3: CardDef = CardDef { keywords: Keywords::of(&[]), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], enters: None, additional_cost: Some(Cost::NONE.and(Cost::mana(ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }))), ..CardDef::BLANK };
#[allow(unused_variables, unused_mut)] fn perpetual_4_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); entering.tapped = true;
 Ok(()) }
static PERPETUAL_4: CardDef = CardDef { keywords: Keywords::of(&[]), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], enters: Some(perpetual_4_enters), ..CardDef::BLANK };
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| ((q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Creature)) && (i64::from(q.mana_value(id))) <= (3i64))).collect::<Vec<ObjectId>>() }; let moved_0 = g.exile_all(&objects_0);
g.end_sentence();
planned!("engine-no-perpetual-effects" => let objects_2: Vec<ObjectId> = { let q: &dyn Query = g; moved_0.clone() };
for &object in &objects_2 { let kind_2_0 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::Rules(&PERPETUAL_3) }; cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_2_0); let kind_2_1 = { let q: &dyn Query = g; cantrip_mechanics::perpetual::Perpetual::Rules(&PERPETUAL_4) }; cantrip_mechanics::perpetual::apply(g, asker.source, object, kind_2_1); });
g.end_sentence();
for bound_0 in { let q: &dyn Query = g; moved_0.clone() } {
cantrip_api::instruction(g, |g| { let cards_7_0 = { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<ObjectId>>() }; let players_7_0 = { let q: &dyn Query = g; Some(bound_0).map(|o: ObjectId| q.state().obj(o).owner).into_iter().collect::<Vec<PlayerId>>() }; cantrip_api::instruction(g, |g| { for &card in &cards_7_0 { if g.obj(card).zone.is(ZoneKind::Exile) { for &player in &players_7_0 { g.allow_play(card, player, PlayGrant { lands: true, expiry: Expiry::Never, ..PlayGrant::CAST }); } } } });
 });
g.end_sentence();
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Divine Purge", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [2, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
