// Price of Progress
// status: ready
// Price of Progress deals damage to each player equal to twice the number of nonbasic lands that player controls.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() };
let mut gathered_1: Vec<(ObjectId, Target, u16)> = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let source_2 = { let q: &dyn Query = g; Some(asker.source) }; let amount_2 = { let q: &dyn Query = g; ((2i64 * (q.state().battlefield().iter().copied().filter(|&id| (!(q.chars(id).supertypes.basic) && q.chars(id).is(CardType::Land) && Some(bound_0) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).len() as i64)).clamp(0, 65535) as u16 }; if let Some(source) = source_2 { for t in { let q: &dyn Query = g; (Some(bound_0).into_iter().collect::<Vec<PlayerId>>()).into_iter().map(Target::Player).collect::<Vec<Target>>() } { gathered_1.push((source, t, amount_2)); } }
    } 
g.damage_together(&gathered_1);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Price of Progress", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
