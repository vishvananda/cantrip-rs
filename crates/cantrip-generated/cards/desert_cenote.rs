// Desert Cenote
// status: planned
// requires: mana of a color not chosen as it entered
// requires: operation ChooseValueWith
// If you were the starting player, this land enters tapped.
// As this land enters, choose up to two colors from among cards in your hand.
// {T}: Add {C}.
// {T}: Add one mana of a chosen color.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); if { let q: &dyn Query = g; Target::Object(asker.source).object().map(|o: ObjectId| q.controller(o)).is_some_and(|player: PlayerId| q.state().turn.starting == player) } {
entering.tapped = true;
}
 Ok(()) }
#[allow(unused_variables, unused_mut)] fn ability_1_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); let chosen_1 = planned_value!("operation ChooseValueWith", Color::White => cantrip_mechanics::choose::value(g, { let q: &dyn Query = g; Target::Object(asker.source).object().map(|o: ObjectId| q.controller(o)).into_iter().collect::<Vec<PlayerId>>() }, asker.source, ChoiceKind::Color, ChoiceTerms { options: &[ChoosableColor::ColorsInPlayersHand({ let q: &dyn Query = g; Target::Object(asker.source).object().map(|o: ObjectId| q.controller(o)) })], except: &[], count: Some({ let q: &dyn Query = g; (2i64).max(0) as u32 }), secret: false, unchosen: false, random: false, ordered: false, up_to: true })?);
 Ok(()) }
fn ability_2_mana(g: &mut dyn Game, asker: &Asker, chosen: Option<Color>) -> Result<(),Suspend> { let mana_players_2 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_2_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
for player in mana_players_2 {
g.add_mana(player, Mana::C, mana_2_0);}
 Ok(()) }
fn entering_all(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { ability_0_enters(g, entering)?; ability_1_enters(g, entering)?;  Ok(()) }
planned_item!("mana-of-a-color-not-chosen-as-it-entered", "(kind:Activated,args:(origin:Ordinary,cost:(parts:[TapSymbol]),limits:(once:false,once_each_turn:false,sorcery:false,condition:None,activators:None,keyword:None,modifiers:[]),announcement:(targets:[],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:AddMana,args:(players:(kind:One,args:(kind:AbilityController)),mana:Amount(color:Chosen,amount:(kind:Constant,args:1)),restriction:None))))])))");

pub static CARD: CardDef = CardDef { name: "Desert Cenote", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Land]), subtypes: SubtypeSet::of(&[Subtype::Desert]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), Adds { pool: ManaPool([0, 0, 0, 0, 0, 1]), ..add_by(ability_2_mana) }, "{T}: Add {C}.")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(entering_all), modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
