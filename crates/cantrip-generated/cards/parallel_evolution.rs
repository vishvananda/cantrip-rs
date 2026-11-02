// Parallel Evolution
// status: ready
// For each creature token on the battlefield, its controller creates a token that's a copy of that creature.
// Flashback {4}{G}{G}{G} (You may cast this card from your graveyard for its flashback cost. Then exile it.)
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut gathered_1: Vec<(Copiable, PlayerId)> = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let player_2 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let originals_2 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).is(CardType::Creature) && q.state().obj(id).token && Some(bound_0) == Some(q.controller(id)))).collect::<Vec<ObjectId>>() }; for original in originals_2 { if let Some(values) = { let q: &dyn Query = g; Some(original).map(|original: ObjectId| q.copiable_values(original)) } { gathered_1.push((values, player_2)); } }
    } 
let created_1 = g.create_token_copies_together(&gathered_1)?;
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_require(q: &dyn Query, asker: &Asker) -> bool { ((q.chars(asker.source).is(CardType::Instant) || q.chars(asker.source).is(CardType::Sorcery))) }

pub static CARD: CardDef = CardDef { name: "Parallel Evolution", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 2, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ways: &[way(Cost::NONE.and(Cost::mana(ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 0, 3, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ZoneKind::Graveyard, "Flashback").only_from().exile_after().condition(ability_1_require)], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
