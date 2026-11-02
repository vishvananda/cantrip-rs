// Walking Desecration
// status: planned
// requires: a lasting rule reading what its ability chose
// {B}, {T}: Creatures of the creature type of your choice attack this turn if able.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let chosen_0 = cantrip_mechanics::chosen_type::creature_type_now(g, player_0, asker.source)?;
g.end_sentence();
planned!("a-lasting-rule-reading-what-its-ability-chose", CreateContinuousEffect, "affected" = "{\"Objects\":{\"args\":{\"args\":[{\"args\":\"Creature\",\"kind\":\"CardType\"},{\"args\":\"CreatureType\",\"kind\":\"Chosen\"}],\"kind\":\"All\"},\"kind\":\"Battlefield\"}}", "changes" = "[{\"args\":{\"RequireCombat\":{\"action\":{\"Attack\":\"Any\"},\"amount\":{\"AtLeast\":{\"args\":1,\"kind\":\"Constant\"}}}},\"kind\":\"Rule\"}]", "duration" = "\"EndOfTurn\"", "evaluation" = "\"LiveObjects\"", "source" = { let q: &dyn Query = g; Some(asker.source) });
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Walking Desecration", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Zombie]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), ability_0, "{B}, {T}: Creatures of the creature type of your choice attack this turn if able.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
