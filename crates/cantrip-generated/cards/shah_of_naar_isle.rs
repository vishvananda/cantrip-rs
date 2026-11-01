// Shah of Naar Isle
// status: planned
// requires: trigger event Object EchoPaid
// Trample
// Echo {0} (At the beginning of your upkeep, if this came under your control since the beginning of your last upkeep, sacrifice it unless you pay its echo cost.)
// When this creature's echo cost is paid, each opponent may draw up to three cards.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
let mut each_2 = vec![Default::default(); players_0.len()];
let mut done_total_2: i64 = 0;
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let most_1 = { let q: &dyn Query = g; (3i64).max(0) as u32 }; let mut done_1: i64 = 0; for player in { let q: &dyn Query = g; Some(bound_0).into_iter().collect::<Vec<PlayerId>>() } { let n = cantrip_mechanics::choose::number(g, player, asker.source, 0, most_1, "Draw")?; let before = g.history().drawn(player) as i64; g.draw(player, n); done_1 += g.history().drawn(player) as i64 - before; }
 each_2[index_0] = done_1;
done_total_2 += done_1;
   } 
g.end_sentence();
 Ok(()) }
planned_item!("trigger-event-object-echo-paid" => matcher: #[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::EchoPaid { permanent, .. } = event { let id = *permanent; let subject = Target::Object(id); let data = [u32::MAX, u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None });

pub static CARD: CardDef = CardDef { name: "Shah of Naar Isle", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Efreet]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(6), toughness: Some(6), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Trample]), protection: ColorSet(0), spell: None, abilities: &[cantrip_mechanics::echo!(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })), "Echo"),triggered(when_asked(planned_value!("trigger event Object EchoPaid", EventMask(0) => EventMask(EventMask::ECHO_PAID.0)), ability_2_event), ability_2, "When this creature's echo cost is paid, each opponent may draw up to three cards.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
