// Slobad, Iron Goblin
// status: ready
// {T}, Sacrifice an artifact: Add an amount of {R} equal to the sacrificed artifact's mana value. Spend this mana only to cast artifact spells or activate abilities of artifacts.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_mana(g: &mut dyn Game, asker: &Asker, chosen: Option<Color>) -> Result<(),Suspend> { let mana_players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_0_0 = { let q: &dyn Query = g; ((q.events().iter().rev().find_map(|r| match r.event { Event::ZoneChange { old, from, part: false, .. } if from.is(ZoneKind::Battlefield) => Some(old), _ => None }).into_iter().collect::<Vec<ObjectId>>()).get(0).copied().map_or(0i64, |o: ObjectId| i64::from(q.mana_value(o)))) as u32 };
for player in mana_players_0 {
g.add_restricted_mana(player, Mana::R, mana_0_0, ManaRestriction::ARTIFACT_SPELLS_OR_ABILITIES_OF_ARTIFACTS);}
 Ok(()) }
#[allow(unused_variables)] fn ability_0_1(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { q.chars(id).is(CardType::Artifact) }

pub static CARD: CardDef = CardDef { name: "Slobad, Iron Goblin", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Phyrexian,Subtype::Goblin,Subtype::Artificer]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }).and(sacrifice(1, ability_0_1)), add_by(ability_0_mana).only_for(ManaRestriction::ARTIFACT_SPELLS_OR_ABILITIES_OF_ARTIFACTS), "{T}, Sacrifice an artifact: Add an amount of {R} equal to the sacrificed artifact's mana value. Spend this mana only to cast artifact spells or activate abilities of artifacts.")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
