// Ice Cave
// status: ready
// Whenever a player casts a spell, any other player may pay that spell's mana cost. If a player does, counter the spell. (Mana cost includes color.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if true && true { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut paid_0 = false; let mut who_0: Option<PlayerId> = None; for bound_1000001 in { let q: &dyn Query = g; { let other = Some(PlayerId(item.data[0] as u8)); q.state().apnap().filter(|p| Some(*p) != other).collect::<Vec<PlayerId>>() } } {
let payer_2 = { let q: &dyn Query = g; Some(bound_1000001).expect("a reference that always names a player") }; let paid_2 = match { let q: &dyn Query = g; item.subject.object() } { Some(o) if g.card(g.obj(o).card).has_cost => { let price = of_mana(g.card(g.obj(o).card).cost); g.may_pay(payer_2, asker.source, o, price, &PAY_QUESTION)? } _ => false };
if paid_2 { paid_0 = true; who_0 = Some(bound_1000001); break; } }
g.end_sentence();
if { let q: &dyn Query = g; ((paid_0 as i64) != 0) } {
let objects_3 = { let q: &dyn Query = g; item.subject.object().into_iter().collect::<Vec<ObjectId>>() }; cantrip_api::instruction(g, |g| { for spell in objects_3 { g.counter(spell); } });
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
static PAY_QUESTION: Question = Question::new("Pay", "pay {cost}?", "", "Pay {cost}", "Decline");

pub static CARD: CardDef = CardDef { name: "Ice Cave", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::SPELL_CAST.0), ability_0_event), ability_0, "Whenever a player casts a spell, any other player may pay that spell's mana cost. If a player does, counter the spell.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
