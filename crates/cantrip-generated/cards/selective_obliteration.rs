// Selective Obliteration
// status: planned
// requires: object filter ChosenBy
// Each player chooses a color. Then exile each permanent unless it's colorless or it's only the color its controller chose.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let player_1 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let Answer::Colors(colors_1) = g.ask(player_1, RequestKind::ChooseColors { source: asker.source, choices: ColorSet(31), count: 1, purpose: ColorUse::Any })? else { return Ok(()) }; let chosen_1 = colors_1[0];
    } 
g.end_sentence();
let objects_2 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| ((q.chars(id).is(CardType::Artifact) || q.chars(id).is(CardType::Battle) || q.chars(id).is(CardType::Creature) || q.chars(id).is(CardType::Enchantment) || q.chars(id).is(CardType::Land) || q.chars(id).is(CardType::Planeswalker)) && !((q.chars(id).colors.is_empty() || ((q.chars(id).colors.iter().count() as i64) == (1i64) && planned_value!("object filter ChosenBy", false => Some(id).map(|o: ObjectId| q.controller(o)).is_some_and(|p: PlayerId| q.chosen_by(asker, p, ChoiceKind::Color).matches(id)))))))).collect::<Vec<ObjectId>>() }; let moved_2 = g.exile_all(&objects_2);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Selective Obliteration", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 0, 2], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
