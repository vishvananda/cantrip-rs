// Vitality Hunter
// status: planned
// requires: the X of the monstrosity a becomes-monstrous trigger reads
// Lifelink
// {X}{W}{W}: Monstrosity X. (If this creature isn't monstrous, put X +1/+1 counters on it and it becomes monstrous.)
// When this creature becomes monstrous, put a lifelink counter on each of up to X target creatures.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (i64::from(item.x)).min(i16::MAX.into()) as i16 }; g.monstrosity(asker.source, amount_0);
g.end_sentence();
 Ok(()) }
planned_item!("the-x-of-the-monstrosity-a-becomes-monstrous-trigger-reads", "(kind:Triggered,args:(event:Object(event:BecomesMonstrous,objects:(kind:SameObject,args:(kind:Source))),intervening_if:None,limits:(per_turn:None,total:None),announcement:(targets:[(slot:(0),kind:Object((kind:Permanent,zones:[Battlefield])),count:UpTo((kind:AnnouncedX)),filter:(kind:CardType,args:Creature),players:None,distribute:None,constraints:[],chooser:None)],modes:None),body:(instructions:[(kind:Effect,args:(result:(0),operation:(kind:PutCounters,args:(placements:[(recipients:(kind:Objects,args:(kind:Targets,args:(0))),kind:Named(LifelinkCounter),amount:(kind:Constant,args:1),selection:None)]))))])))");

pub static CARD: CardDef = CardDef { name: "Vitality Hunter", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [1, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Nightmare]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Lifelink]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 1, symbols: [2, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ability_1, "{X}{W}{W}: Monstrosity X.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
