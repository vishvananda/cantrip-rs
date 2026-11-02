// Hordewing Skaab
// status: planned
// requires: amount of an event its trigger does not carry
// Flying
// Other Zombies you control have flying.
// Whenever one or more Zombies you control deal combat damage to one or more of your opponents, you may draw cards equal to the number of opponents dealt damage this way. If you do, discard that many cards.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (!(Some(asker.source) == Some(affected)) && ((*so_far).has_subtype(q.words(asker.source).subtype(Subtype::Zombie)) && Some(asker.controller) == Some(q.controller(affected)))) }
#[allow(unused_variables)] fn ability_1_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.keywords = chars.keywords.with(Keyword::Flying); }
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Damage { source, target, amount, kind, .. } = event { let (source, amount, combat) = (*source, *amount, kind.is_combat()); let subject = Target::Object(source); let data = [u32::from(amount), match target { Target::Player(p) => u32::from(p.0), _ => u32::MAX }]; if combat && !q.state().obj(source).zone.is(ZoneKind::Stack) && (q.chars(source).has_subtype(q.words(asker.source).subtype(Subtype::Zombie)) && Some(asker.controller) == Some(q.controller(source))) && matches!(*target, Target::Player(p) if (Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>()).contains(&p)) { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_5: Vec<ObjectId> = Vec::new();
let mut hoisted_4: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let payer_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let (paid_0, after_0, before_0, picked_0): (bool, Vec<ObjectId>, Vec<ObjectId>, Vec<ObjectId>) = if cantrip_mechanics::effects::ask_may(g, payer_0, asker.source, Purpose::Optional)? {
let amount_2 = { let q: &dyn Query = g; (planned_value!("amount of an event its trigger does not carry", 0i64 => q.event_total(item))).max(0) as u32 }; let players_2 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mut done_2: i64 = 0; let mut drawn_2: Vec<ObjectId> = Vec::new(); for player in players_2 { let before = g.history().drawn(player) as i64; let held = g.state().zone(Zone::hand(player)).to_vec(); g.draw(player, amount_2); done_2 += g.history().drawn(player) as i64 - before; drawn_2.extend(g.state().zone(Zone::hand(player)).iter().copied().filter(|c| !held.contains(c))); }
let paid_1 = done_2 > 0;
(paid_1, { let q: &dyn Query = g; (drawn_2).clone() }, Vec::new(), Vec::new()) } else { (false, Vec::new(), Vec::new(), Vec::new()) };
g.end_sentence();
if { let q: &dyn Query = g; ((paid_0 as i64) != 0) } {
let amount_3 = { let q: &dyn Query = g; (planned_value!("amount of an event its trigger does not carry", 0i64 => q.event_total(item))).max(0) as u32 }; let players_3: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }.into_iter().collect(); let mut cards_3 = Vec::new(); for player in players_3 { cards_3.extend(cantrip_mechanics::choose_from_hand(g, player, asker.source, amount_3)?); } let discarded_3 = g.discard_together(&cards_3);
g.end_sentence();
hoisted_4.extend(discarded_3.clone());
hoisted_5.extend({ let q: &dyn Query = g; discarded_3.clone() });
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Hordewing Skaab", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Zombie,Subtype::Horror]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::DAMAGE.0), ability_2_event), ability_2, "Whenever one or more Zombies you control deal combat damage to one or more of your opponents, you may draw cards equal to the number of opponents dealt damage this way. If you do, discard that many cards.").targets(&[]).once_per_batch()], statics: &[static_ability(Layer::Ability, ability_1_affects, ability_1_apply_0, "Other Zombies you control have flying.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
