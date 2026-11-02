// Cryptic Spires
// status: planned
// requires: ability BeforeGame
// requires: mana of a color not chosen as it entered
// requires: operation ChooseValueWith
// As you create your deck, circle two of the colors below.
// This land enters tapped.
// {T}: Add one mana of either of the circled colors.
#[allow(unused_imports)] use cantrip_api::*;
planned_item!("ability-before-game" => #[allow(unused_variables)] fn ability_0_before_game(g: &mut dyn Game, asker: &Asker) -> Result<(), Suspend> { cantrip_mechanics::before_game::perform(g, asker, &[|g: &mut dyn Game, asker: &Asker| -> Result<(), Suspend> { let chosen_0 = planned_value!("operation ChooseValueWith", Color::White => cantrip_mechanics::choose::value(g, { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }, asker.source, ChoiceKind::Color, ChoiceTerms { options: &[ChoosableColor::AnyColor], except: &[], count: Some({ let q: &dyn Query = g; (2i64).max(0) as u32 }), secret: false, unchosen: false, random: false, ordered: false, up_to: false })?);
g.end_sentence();
 Ok(()) }]) });
#[allow(unused_variables, unused_mut)] fn ability_1_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); entering.tapped = true;
 Ok(()) }
planned_item!("ability-before-game" => before_game: (BeforeGameMoment::GameBegins, ability_0_before_game));
planned_item!("mana-of-a-color-not-chosen-as-it-entered", "(kind:Activated,args:(origin:Ordinary,cost:(parts:[TapSymbol]),limits:(once:false,once_each_turn:false,sorcery:false,condition:None,activators:None,keyword:None,modifiers:[]),announcement:(targets:[],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:AddMana,args:(players:(kind:One,args:(kind:AbilityController)),mana:Amount(color:Chosen,amount:(kind:Constant,args:1)),restriction:None))))])))");

pub static CARD: CardDef = CardDef { name: "Cryptic Spires", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Land]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_1_enters), modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
