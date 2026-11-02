// Stormbreath Dragon
// status: ready
// Flying, haste, protection from white
// {5}{R}{R}: Monstrosity 3. (If this creature isn't monstrous, put three +1/+1 counters on it and it becomes monstrous.)
// When this creature becomes monstrous, it deals damage to each opponent equal to the number of cards in that player's hand.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (3i64).max(0) as i16 }; g.monstrosity(asker.source, amount_0);
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_4_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Monstrous { permanent } = event { let id = *permanent; let subject = Target::Object(id); let data = [u32::MAX, u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_4(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_1: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
{ let order: Vec<PlayerId> = g.state().apnap().collect(); players_1.sort_by_key(|p| order.iter().position(|o| o == p)); }
let mut gathered_2: Vec<(ObjectId, Target, u16)> = Vec::new();
 for (index_1, bound_0) in players_1.iter().copied().enumerate() { let source_3 = { let q: &dyn Query = g; item.subject.object() }; let amount_3 = { let q: &dyn Query = g; ((((Some(bound_0).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).len() as i64).clamp(0, 65535) as u16 }; if let Some(source) = source_3 { for t in { let q: &dyn Query = g; (Some(bound_0).into_iter().collect::<Vec<PlayerId>>()).into_iter().map(Target::Player).collect::<Vec<Target>>() } { gathered_2.push((source, t, amount_3)); } }
    } 
g.damage_together(&gathered_2);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Stormbreath Dragon", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Dragon]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(4), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::Haste]), protection: ColorSet(1), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 5, x: 0, symbols: [0, 0, 0, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), ability_3, "{5}{R}{R}: Monstrosity 3.").targets(&[]),triggered(when_asked(EventMask(EventMask::MONSTROUS.0), ability_4_event), ability_4, "When this creature becomes monstrous, it deals damage to each opponent equal to the number of cards in that player's hand.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
