// Eclipsed Realms
// status: planned
// requires: a chosen type mana rule that names creatures it doesn't print
// As this land enters, choose Elemental, Elf, Faerie, Giant, Goblin, Kithkin, Merfolk, or Treefolk.
// {T}: Add {C}.
// {T}: Add one mana of any color. Spend this mana only to cast a spell of the chosen type or activate an ability of a source of the chosen type.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); let Some(chosen_0) = cantrip_mechanics::chosen_type::choose_creature_type(g, entering)? else { return Ok(()) };
 Ok(()) }
fn ability_1_mana(g: &mut dyn Game, asker: &Asker, chosen: Option<Color>) -> Result<(),Suspend> { let mana_players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_1_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
for player in mana_players_1 {
g.add_mana(player, Mana::C, mana_1_0);}
 Ok(()) }
planned_item!("a-chosen-type-mana-rule-that-names-creatures-it-doesn-t-print", "(kind:Activated,args:(origin:Ordinary,cost:(parts:[TapSymbol]),limits:(once:false,once_each_turn:false,sorcery:false,condition:None,activators:None,keyword:None,modifiers:[]),announcement:(targets:[],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:AddMana,args:(players:(kind:One,args:(kind:AbilityController)),mana:ChooseOne([Amount(color:White,amount:(kind:Constant,args:1)),Amount(color:Blue,amount:(kind:Constant,args:1)),Amount(color:Black,amount:(kind:Constant,args:1)),Amount(color:Red,amount:(kind:Constant,args:1)),Amount(color:Green,amount:(kind:Constant,args:1))]),restriction:Some(Either([CastSpell((kind:All,args:[(kind:CardType,args:Creature),(kind:Chosen,args:CreatureType)])),ActivateAbility(sources:(kind:All,args:[(kind:CardType,args:Creature),(kind:Chosen,args:CreatureType)]),permanents_only:false)]))))))])))");

pub static CARD: CardDef = CardDef { name: "Eclipsed Realms", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Land]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), Adds { pool: ManaPool([0, 0, 0, 0, 0, 1]), ..add_by(ability_1_mana) }, "{T}: Add {C}.")], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_0_enters), modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
