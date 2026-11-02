// Soul Shredder
// status: planned
// requires: one or more that die read as each in the data
// Haste
// Whenever one or more other creatures die, Soul Shredder perpetually gets +1/+1. This ability also triggers if this card is in your graveyard.
// Crew 1
// {1}, Sacrifice two creatures: Return this card from your graveyard to the battlefield. Activate only as a sorcery.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let objects_0 = ({ let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }).into_iter().filter(|&o: &ObjectId| !g.state().obj(o).token).collect::<Vec<ObjectId>>(); let moved_0 = { let rows_0: Vec<(ObjectId, PlayerId, bool)> = objects_0.iter().map(|&o| (o, player_0, false)).collect(); let came_0 = g.put_all_onto_battlefield_as(&rows_0)?; came_0.into_iter().filter(|&o| g.state().obj(o).zone.is(ZoneKind::Battlefield)).collect::<Vec<ObjectId>>() };
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_3_1(q: &dyn Query, asker: &Asker, id: ObjectId) -> bool { q.chars(id).is(CardType::Creature) }
planned_item!("one-or-more-that-die-read-as-each-in-the-data", "(kind:Scoped,args:(zones:[Graveyard,Battlefield],condition:None,ability:(kind:Triggered,args:(event:ZoneChange(objects:(kind:All,args:[(kind:Not,args:(kind:SameObject,args:(kind:Source))),(kind:CardType,args:Creature)]),from:Some(Battlefield),to:Graveyard),intervening_if:None,limits:(per_turn:None,total:None),announcement:(targets:[],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:EffectOn,args:(subject:Objects((kind:Union,args:[(kind:One,args:(kind:Source)),(kind:One,args:(kind:Source))])),effects:[(kind:Term,args:(family:\"PerpetualEffect\",form:\"AdjustPT\",operands:[(kind:Integer,args:1),(kind:Integer,args:1)]))],until:None))))])))))");

pub static CARD: CardDef = CardDef { name: "Soul Shredder", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact]), subtypes: SubtypeSet::of(&[Subtype::Vehicle]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(2), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Haste]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::crew::crew(1, "Crew 1"),ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(sacrifice(2, ability_3_1)), ability_3, "{1}, Sacrifice two creatures: Return this card from your graveyard to the battlefield. Activate only as a sorcery.").targets(&[]).sorcery_speed().from(ZoneKind::Graveyard)], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
