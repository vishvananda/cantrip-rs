// Restless Cottage
// status: planned
// requires: a target from a graveyard, where the card says any
// This land enters tapped.
// {T}: Add {B} or {G}.
// {2}{B}{G}: This land becomes a 4/4 black and green Horror creature until end of turn. It's still a land.
// Whenever this land attacks, create a Food token and exile up to one target card from a graveyard.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables, unused_mut)] fn ability_0_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); entering.tapped = true;
 Ok(()) }
fn ability_1_mana(g: &mut dyn Game, asker: &Asker, chosen: Option<Color>) -> Result<(),Suspend> { let mana_players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_1 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
for player in mana_players_1 {
let color = chosen.expect("mana activation selected a color");
g.add_mana(player, color.mana(), mana_1);}
 Ok(()) }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_2 = { let q: &dyn Query = g; (Some(asker.source).into_iter().collect::<Vec<ObjectId>>()).into_iter().filter(|&o| q.state().obj(o).became.is_none()).collect::<Vec<ObjectId>>() }; let source_2 = { let q: &dyn Query = g; Some(asker.source) }; let duration_2 = Duration::EndOfTurn;
let kind_2_0_0 = { let q: &dyn Query = g; EffectKind::SetPt((4i64) as i16, (4i64) as i16) };
let kind_2_1_0 = { let q: &dyn Query = g; EffectKind::SetColors(q.words(asker.source).colors(ColorSet(20))) };
let kind_2_2_0 = { let q: &dyn Query = g; EffectKind::AddSubtype(q.words(asker.source).subtype(Subtype::Horror)) };
let kind_2_3_0 = { let q: &dyn Query = g; EffectKind::AddTypes(TypeSet::of(&[CardType::Creature])) };
if let Some(source) = source_2 { cantrip_api::instruction(g, |g| { for &object in &objects_2 { g.add_effect(source, object, kind_2_0_0, duration_2); g.add_effect(source, object, kind_2_1_0, duration_2); g.add_effect(source, object, kind_2_2_0, duration_2); g.add_effect(source, object, kind_2_3_0, duration_2);  } }); }
g.end_sentence();
 Ok(()) }
planned_item!("a-target-from-a-graveyard-where-the-card-says-any", "(kind:Triggered,args:(event:Object(event:Attacks,objects:(kind:SameObject,args:(kind:Source))),intervening_if:None,limits:(per_turn:None,total:None),announcement:(targets:[(slot:(0),kind:Object((kind:Card,zones:[Graveyard])),count:UpTo((kind:Constant,args:1)),filter:(kind:InZone,args:Players(zone:Graveyard,players:(kind:One,args:(kind:AbilityController)))),players:None,distribute:None,constraints:[],chooser:None)],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:CreateTokens,args:(players:(kind:One,args:(kind:AbilityController)),tokens:[(amount:(kind:Constant,args:1),words:Rules,recipe:Defined((name:FromSubtypes,colors:[],chosen_color:false,chosen_creature_type:false,supertypes:[],card_types:[Artifact],subtypes:[Food],power_toughness:None,abilities:[(kind:Activated,args:(origin:Ordinary,cost:(parts:[Mana([Generic(2)]),TapSymbol,Sacrifice(objects:(kind:One,args:(kind:Source)),choose:None)]),limits:(once:false,once_each_turn:false,sorcery:false,condition:None,activators:None,keyword:None,modifiers:[]),announcement:(targets:[],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:GainLife,args:(players:(kind:One,args:(kind:AbilityController)),amount:(kind:Constant,args:3)))))])))],abilities_of:None)))],entry:(tapped:false,counters:[],states:[]))))),(kind:Effect,args:(result:(1),operation:(kind:Move,args:(objects:(kind:One,args:(kind:Target,args:(slot:(0),index:0))),destination:Exile,entry:(tapped:false,counters:[],states:[]),face_down:false))))])))");

pub static CARD: CardDef = CardDef { name: "Restless Cottage", token_words: &[], oracle: true, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Land]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[mana_ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), add_by(ability_1_mana).choose_color().colors(ColorSet(20)).yields_any(1), "{T}: Add {B} or {G}."),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 2, x: 0, symbols: [0, 0, 1, 0, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ability_2, "{2}{B}{G}: This land becomes a 4/4 black and green Horror creature until end of turn. It's still a land.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_0_enters), modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
