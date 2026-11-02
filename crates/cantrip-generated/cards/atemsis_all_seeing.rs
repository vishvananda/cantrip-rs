// Atemsis, All-Seeing
// status: ready
// Flying
// {2}{U}, {T}: Draw two cards, then discard a card.
// Whenever Atemsis deals damage to an opponent, you may reveal your hand. If cards with at least six different mana values are revealed this way, that player loses the game.
#[allow(unused_imports)] use cantrip_api::*;
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let amount_0 = { let q: &dyn Query = g; (2i64).max(0) as u32 }; let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_0 { g.draw(player, amount_0); }
g.end_sentence();
let amount_1 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_1: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }.into_iter().collect(); let mut cards_1 = Vec::new(); for player in players_1 { cards_1.extend(cantrip_mechanics::choose_from_hand(g, player, asker.source, amount_1)?); } let discarded_1 = g.discard_together(&cards_1);
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Damage { source, target, amount, kind, .. } = event { let (source, amount, combat) = (*source, *amount, kind.is_combat()); let subject = Target::Object(source); let data = [u32::from(amount), match target { Target::Player(p) => u32::from(p.0), _ => u32::MAX }]; if !q.state().obj(source).zone.is(ZoneKind::Stack) && Some(asker.source) == Some(source) && matches!(*target, Target::Player(p) if (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&p)) { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_4: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let payer_2 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") };
let paid_2 = !({ let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>() }).is_empty() && cantrip_mechanics::effects::ask_may(g, payer_2, asker.source, Purpose::Optional)?; if paid_2 {
let objects_3 = { let q: &dyn Query = g; ((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>() }; let mut owners_3: Vec<PlayerId> = objects_3.iter().map(|o| g.state().obj(*o).owner).collect(); owners_3.dedup(); for owner in owners_3 { let cards: Vec<ObjectId> = objects_3.iter().copied().filter(|o| g.state().obj(*o).owner == owner).collect(); g.reveal(owner, &cards); }
g.end_sentence();
hoisted_4.extend(objects_3.clone());
}
g.end_sentence();
if { let q: &dyn Query = g; ((paid_2 as i64) != 0) } {
if { let q: &dyn Query = g; { let mut values: Vec<i64> = ((((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true).collect::<Vec<ObjectId>>()).into_iter().map(|id: ObjectId| i64::from(q.mana_value(id))).collect(); values.sort_unstable(); values.dedup(); (values.len() as i64) >= (6i64) } } {
{ let player = { let q: &dyn Query = g; Some(PlayerId(item.data[1] as u8)).expect("a reference that always names a player") }; g.lose_game(player); if g.state().is_over() { return Ok(()); }; }
g.end_sentence();
}
g.end_sentence();
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Atemsis, All-Seeing", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 3, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Sphinx]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(4), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 2, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }), ability_1, "{2}{U}, {T}: Draw two cards, then discard a card.").targets(&[]),triggered(when_asked(EventMask(EventMask::DAMAGE.0), ability_2_event), ability_2, "Whenever Atemsis deals damage to an opponent, you may reveal your hand. If cards with at least six different mana values are revealed this way, that player loses the game.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
