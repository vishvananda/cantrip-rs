// Nardole, Resourceful Cyborg
// status: ready
// {T}: Add {U} for each counter on Nardole. Spend this mana only to cast noncreature spells.
// Undying (When this creature dies, if it had no +1/+1 counters on it, return it to the battlefield under its owner's control with a +1/+1 counter on it.)
// Doctor's companion (You can have two commanders if the other is the Doctor.)
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_mana(g: &mut dyn Game, asker: &Asker, chosen: Option<Color>) -> Result<(),Suspend> { let mana_players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_0_0 = { let q: &dyn Query = g; ((Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().map(|o: ObjectId| CounterKind::ALL.iter().map(|k| i64::from(q.state().obj(o).counters(*k))).sum::<i64>()).sum::<i64>()) as u32 };
for player in mana_players_0 {
g.add_restricted_mana(player, Mana::U, mana_0_0, ManaRestriction::NONCREATURE_SPELLS);}
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Nardole, Resourceful Cyborg", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Scientist]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(1), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), add_by(ability_0_mana).only_for(ManaRestriction::NONCREATURE_SPELLS), "{T}: Add {U} for each counter on Nardole. Spend this mana only to cast noncreature spells."),cantrip_mechanics::undying::trigger("Undying")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
