// Doomsday Confluence
// status: planned
// requires: a count of modes a spell or an activated ability reads
// Choose X. You may choose the same mode more than once.
// • Each player sacrifices a nonartifact creature of their choice.
// • Create a 3/3 black Dalek artifact creature token with menace.
// • Each opponent discards a card.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0_mode_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| (!(q.chars(id).is(CardType::Artifact)) && q.chars(id).is(CardType::Creature))).collect::<Vec<ObjectId>>() }; let players_0 = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() }; let mut picks_0: Vec<(PlayerId, ObjectId)> = Vec::new();
let count_0 = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }; for player in players_0 { let eligible: Vec<ObjectId> = objects_0.iter().copied().filter(|&o| g.controller(o) == player && g.can_sacrifice(player, o)).collect(); let taken = match count_0.clone() { cantrip_mechanics::select::Count::Exactly(n) | cantrip_mechanics::select::Count::AsManyAsCan(n) if eligible.len() <= n as usize => eligible, _ => cantrip_mechanics::choose::select(g, player, asker.source, Purpose::Sacrifice, eligible, cantrip_mechanics::select::Selection::of(count_0.clone(), &[]))? }; for object in taken { picks_0.push((player, object)); } }
let sacrificed_0 = g.sacrifice_each(&picks_0);
g.end_sentence();
 Ok(()) }
pub mod token_2 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Dalek).name, token_words: token!(Dalek).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Artifact,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Dalek]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(4)), power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Menace]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_0_mode_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_1 = Vec::new();
let count_1_0 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let def_1_0 = g.card_id(&token_2::CARD);
for player in players_1 {
tokens_1.extend(std::iter::repeat((def_1_0,player)).take(count_1_0 as usize));
} let words_1 = g.words(asker.source);
let created_1 = g.create_tokens_worded(&tokens_1, false, None, words_1)?;
g.end_sentence();
 Ok(()) }
fn ability_0_mode_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_3 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_3: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() }.into_iter().collect(); let mut cards_3 = Vec::new(); for player in g.state().apnap().filter(|p| players_3.contains(p)).collect::<Vec<PlayerId>>() { cards_3.extend(cantrip_mechanics::choose_from_hand(g, player, asker.source, amount_3)?); } let discarded_3 = g.discard_together(&cards_3);
g.end_sentence();
 Ok(()) }
planned_item!("a-count-of-modes-a-spell-or-an-activated-ability-reads", "(kind:Spell,args:(announcement:(targets:[],modes:Some((minimum:(kind:AnnouncedX),maximum:(kind:AnnouncedX),repeat:true,modes:[(announcement:(targets:[],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:Sacrifice,args:(players:(kind:All),objects:(kind:Battlefield,args:(kind:All,args:[(kind:Not,args:(kind:CardType,args:Artifact)),(kind:CardType,args:Creature)])),choose:Some(AsManyAsCan((kind:Constant,args:1)))))))]),weight:1,requires:None),(announcement:(targets:[],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:CreateTokens,args:(players:(kind:One,args:(kind:AbilityController)),tokens:[(amount:(kind:Constant,args:1),words:CreatingEffect,recipe:Defined((name:FromSubtypes,colors:[Black],chosen_color:false,chosen_creature_type:false,supertypes:[],card_types:[Artifact,Creature],subtypes:[Dalek],power_toughness:Some(((kind:Constant,args:3),(kind:Constant,args:3))),abilities:[(kind:Keyword,args:(kind:Menace))],abilities_of:None)))],entry:(tapped:false,counters:[],states:[])))))]),weight:1,requires:None),(announcement:(targets:[],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:Discard,args:(players:(kind:OpponentsOf,args:(kind:AbilityController)),amount:(kind:Constant,args:1),random:false))))]),weight:1,requires:None)],implied:false,unchosen:None,surcharge:None,distinct_targets:false,chooser:None,random:false,budget:None,may_decline:false))),body:(instructions:[])))");

pub static CARD: CardDef = CardDef { name: "Doomsday Confluence", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 2, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_2::CARD];
