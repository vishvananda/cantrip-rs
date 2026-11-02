// Ezio Auditore da Firenze
// status: ready
// Menace
// Assassin spells you cast have freerunning {B}{B}. (You may cast a spell for its freerunning cost if you dealt combat damage to a player this turn with an Assassin or commander.)
// Whenever Ezio deals combat damage to a player, you may pay {W}{U}{B}{R}{G} if that player has 10 or less life. When you do, that player loses the game.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (q.state().obj(affected).became.is_none() && q.state().obj(affected).zone.kind() == ZoneKind::Stack) && ((*so_far).has_subtype(q.words(asker.source).subtype(Subtype::Assassin)) && (q.state().obj(affected).cast_from.is_some() && q.state().stack.iter().any(|i| i.kind == StackKind::Spell && i.object == affected && Some(asker.controller) == Some(i.controller)))) }
#[allow(unused_variables)] fn ability_1_0_way_cost(q: &dyn Query, asker: &Asker, card: ObjectId, player: PlayerId) -> Option<Cost> { (player == asker.controller && ((q.chars(card).has_subtype(q.words(asker.source).subtype(Subtype::Assassin)))) && cantrip_mechanics::freerunning::assassin_dealt_combat_damage(q, asker)).then(|| Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 2, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })) }
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Damage { source, target, amount, kind, .. } = event { let (source, amount, combat) = (*source, *amount, kind.is_combat()); let subject = Target::Object(source); let data = [u32::from(amount), match target { Target::Player(p) => u32::from(p.0), _ => u32::MAX }]; if combat && !q.state().obj(source).zone.is(ZoneKind::Stack) && Some(asker.source) == Some(source) && matches!(*target, Target::Player(p) if true) { return Some(Fired { subject, data }); } }
 None }
fn reflexive_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
if let Some(present_6_0) = { let q: &dyn Query = g; item.subject.player() } { { let player = { let q: &dyn Query = g; present_6_0 }; g.lose_game(player); if g.state().is_over() { return Ok(()); }; }
 }
g.end_sentence();
 Ok(()) }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
if { let q: &dyn Query = g; (Some(PlayerId(item.data[1] as u8)).map_or(0i64, |p: PlayerId| i64::from(q.state().player(p).life))) <= (10i64) } {
'optional_0: {
let payer_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let paid_1 = g.may_pay(payer_1, asker.source, asker.source, Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [1, 1, 1, 1, 1, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), &PAY_QUESTION)?; if !paid_1 { break 'optional_0; }
if paid_1 && true { g.reflexive_trigger(item.object, item.controller, AbilityRef::delayed(item.ability.card, 0), { let q: &dyn Query = g; Some(PlayerId(item.data[1] as u8)) }.map_or(Target::None, Target::Player)); }
g.end_sentence();
}
g.end_sentence();
}
g.end_sentence();
 Ok(()) }
static PAY_QUESTION: Question = Question::new("Pay", "pay {cost}?", "", "Pay {cost}", "Decline");

pub static CARD: CardDef = CardDef { name: "Ezio Auditore da Firenze", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Human,Subtype::Assassin]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(3), toughness: Some(2), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Menace]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::DAMAGE.0), ability_2_event), ability_2, "Whenever Ezio deals combat damage to a player, you may pay {W}{U}{B}{R}{G} if that player has 10 or less life. When you do, that player loses the game.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[delayed_trigger(reflexive_3, "When you do, that player loses the game.").targets(&[])], enters: None, modal: None, enchant: None, granted_ways: &[GrantedWay { text: "Freerunning", cost: ability_1_0_way_cost, from: ZoneKind::Hand, kind: WayKind::Freerunning, ..GrantedWay::CAST }], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
