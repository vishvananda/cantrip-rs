// Apex Observatory // Eye of Ojer Taq
// status: planned
// requires: a card type shared among the cards used to craft it
// requires: craft materials that must have something in common
// requires: operation EffectOn
// This artifact enters tapped. As it enters, choose a card type shared among two exiled cards used to craft it.
// {T}: The next spell you cast this turn of the chosen type can be cast without paying its mana cost.
// {T}: Add one mana of any color.
// Craft with two that share a card type {6} ({6}, Exile this artifact, Exile the two from among other permanents you control and/or cards from your graveyard: Return this card transformed under its owner's control. Craft only as a sorcery.)
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_mana(g: &mut dyn Game, asker: &Asker, chosen: Option<Color>) -> Result<(),Suspend> { let mana_players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
for player in mana_players_0 {
let color = chosen.expect("mana activation selected a color");
g.add_mana(player, color.mana(), mana_0);}
 Ok(()) }
#[allow(unused_variables)] fn ability_1_1(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { true }
planned_item!("craft-materials-that-must-have-something-in-common" => abilities: cantrip_mechanics::craft::ability(ManaCost { generic: 6, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, &cantrip_mechanics::craft::materials(2, false, ability_1_1, "").together(Group::ShareACardType), "Craft {6}"));

pub static CARD: CardDef = CardDef { name: "Eye of Ojer Taq", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), add_by(ability_0_mana).choose_color().colors(ColorSet(31)).yields_any(1), "{T}: Add one mana of any color.")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, back: Some(&back::CARD), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
pub mod back {
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); entering.tapped = true;
planned!("a-card-type-shared-among-the-cards-used-to-craft-it", ChooseValueWith, "actor" = { let q: &dyn Query = g; Target::Object(asker.source).object().map(|o: ObjectId| q.controller(o)).into_iter().collect::<Vec<PlayerId>>() }, "kind" = "\"CardType\"", "terms" = "{\"count\":(),\"except\":[],\"options\":[{\"args\":{\"args\":{\"filter\":{\"args\":{\"kind\":\"Source\"},\"kind\":\"ExiledWith\"},\"kind\":\"Card\",\"scope\":{\"Shared\":\"Exile\"}},\"kind\":\"Zone\"},\"kind\":\"Objects\"}],\"ordered\":false,\"random\":false,\"secret\":false,\"unchosen\":false,\"up_to\":false}");
 Ok(()) }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("operation-effect-on" => cantrip_mechanics::effects::effect_on_players(g, asker, &{ let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }, &[FuturePlayerEffect::MayCastTheirNextSpellThisTurnWithoutPaying(|q: &dyn Query, asker: &Asker, id: ObjectId| planned_value!("depends on an instruction with a gap", None::<CardType> => q.state().chosen_card_type(q.linked(item))).is_some_and(|chosen| q.chars(id).is(chosen)))], None)?);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Apex Observatory", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), ability_1, "{T}: The next spell you cast this turn of the chosen type can be cast without paying its mana cost.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_0_enters), modal: None, enchant: None, ..CardDef::BLANK };
}
