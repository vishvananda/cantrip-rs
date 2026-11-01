// Urza's Avenger
// status: planned
// requires: a choice of Ability
// requires: a choice of Ability outside its body
// {0}: This creature gets -1/-1 and gains your choice of banding, flying, first strike, or trample until end of turn. (Any creatures with banding, and up to one without, can attack in a band. Bands are blocked as a group. If any creatures with banding you control are blocking or being blocked by a creature, you divide that creature's combat damage, not its controller, among any of the creatures it's being blocked by or is blocking.)
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("a-choice-of-ability" => cantrip_mechanics::choose::value(g, asker.source, { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }, ChoiceKind::Ability, 1, &["Ability(Mechanic { mechanic: Banding, cost: None, amount: None, filter: None })", "Ability(Keyword(Flying))", "Ability(Keyword(FirstStrike))", "Ability(Keyword(Trample))"])?);
g.end_sentence();
planned!("a-choice-of-ability-outside-its-body" => cantrip_mechanics::effects::grant(g, item, { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }, &[Grant::Pump({ let q: &dyn Query = g; -1i64 }, { let q: &dyn Query = g; -1i64 }), Grant::ChosenAbility], Duration::EndOfTurn)?);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Urza's Avenger", token_words: &[], oracle: true, cost: ManaCost { generic: 6, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Shapeshifter]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ability_0, "{0}: This creature gets -1/-1 and gains your choice of banding, flying, first strike, or trample until end of turn.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
