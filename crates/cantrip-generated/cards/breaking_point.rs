// Breaking Point
// status: ready
// Any player may have Breaking Point deal 6 damage to them. If no one does, destroy all creatures. Creatures destroyed this way can't be regenerated.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_6: Vec<ObjectId> = Vec::new();
let mut hoisted_5: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let mut paid_0 = false; let mut who_0: Option<PlayerId> = None; for bound_1000001 in { let q: &dyn Query = g; q.state().apnap().collect::<Vec<PlayerId>>() } {
let payer_2 = { let q: &dyn Query = g; Some(bound_1000001).expect("a reference that always names a player") };
let paid_2 = cantrip_mechanics::effects::ask_may(g, payer_2, asker.source, Purpose::Optional)?; if paid_2 {
let source_3 = { let q: &dyn Query = g; Some(asker.source) }; let amount_3 = { let q: &dyn Query = g; (6i64).max(0) as u16 }; let recipients_3 = { let q: &dyn Query = g; (Some(bound_1000001).into_iter().collect::<Vec<PlayerId>>()).into_iter().map(Target::Player).collect::<Vec<Target>>() }; if let Some(source) = source_3 { let batch: Vec<(ObjectId, Target, u16)> = recipients_3.iter().map(|t| (source, *t, amount_3)).collect(); g.damage_together(&batch); }
g.end_sentence();
}
if paid_2 { paid_0 = true; who_0 = Some(bound_1000001); break; } }
g.end_sentence();
if { let q: &dyn Query = g; !(((paid_0 as i64) != 0)) } {
let objects_4 = { let q: &dyn Query = g; q.state().battlefield().iter().copied().filter(|&id| q.chars(id).is(CardType::Creature)).collect::<Vec<ObjectId>>() }; let destroyed_4 = g.destroy_all_no_regen(&objects_4);
g.end_sentence();
hoisted_5.extend(destroyed_4.clone());
hoisted_6.extend({ let q: &dyn Query = g; objects_4.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() });
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Breaking Point", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Sorcery]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
