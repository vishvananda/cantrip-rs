// Miss Highwater
// status: planned
// requires: counter kind Contract
// requires: counters on a player
// requires: duration WhilePlayerHasCounter
// Menace
// Whenever Miss Highwater deals combat damage to a player who doesn't have a contract counter, they may discard their hand. If they do, they draw seven cards and get a contract counter. For as long as they have a contract counter, when they lose the game, for each artifact and creature they controlled, create a token that's a copy of it.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Damage { source, target, amount, kind, .. } = event { let (source, amount, combat) = (*source, *amount, kind.is_combat()); let subject = Target::Object(source); let data = [u32::from(amount), match target { Target::Player(p) => u32::from(p.0), _ => u32::MAX }]; if combat && !q.state().obj(source).zone.is(ZoneKind::Stack) && Some(asker.source) == Some(source) && matches!(*target, Target::Player(p) if (q.state().apnap().filter(|&bound_0: &PlayerId| (planned_value!("counters on a player", 0i64 => (Some(bound_0).into_iter().collect::<Vec<PlayerId>>()).into_iter().map(|p: PlayerId| i64::from(q.state().player(p).counters(Some(planned_value!("counter kind Contract", CounterKind::Charge => CounterKind::Contract))))).sum::<i64>())) == (0i64)).collect::<Vec<PlayerId>>()).contains(&p)) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let mut hoisted_7: Vec<ObjectId> = Vec::new();
let mut hoisted_3: Vec<ObjectId> = Vec::new();
let mut hoisted_2: Vec<ObjectId> = Vec::new();
let asker = &Asker::new(item.object, item.controller);
let payer_0 = { let q: &dyn Query = g; Some(PlayerId(item.data[1] as u8)).expect("a reference that always names a player") };
let paid_0 = cantrip_mechanics::effects::ask_may(g, payer_0, asker.source, Purpose::Optional)?; if paid_0 {
let amount_1 = { let q: &dyn Query = g; ((((Some(PlayerId(item.data[1] as u8)).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).len() as i64).max(0) as u32 }; let players_1 = { let q: &dyn Query = g; Some(PlayerId(item.data[1] as u8)).into_iter().collect::<Vec<PlayerId>>() }; let mut cards_1 = Vec::new(); for player in players_1 { cards_1.extend(cantrip_mechanics::choose_from_hand(g, player, asker.source, amount_1)?); } let discarded_1 = g.discard_together(&cards_1);
g.end_sentence();
hoisted_2.extend(discarded_1.clone());
hoisted_3.extend({ let q: &dyn Query = g; discarded_1.clone() });
}
g.end_sentence();
if { let q: &dyn Query = g; ((paid_0 as i64) != 0) } {
let amount_4 = { let q: &dyn Query = g; (7i64).max(0) as u32 }; let players_4 = { let q: &dyn Query = g; Some(PlayerId(item.data[1] as u8)).into_iter().collect::<Vec<PlayerId>>() };
for player in players_4 { g.draw(player, amount_4); }
g.end_sentence();
let mut counters_5: Vec<(ObjectId, CounterKind, i16)> = Vec::new();
planned!("counters-on-a-player" => let amount = { let q: &dyn Query = g; (1i64).max(0) as i16 }; for player in { let q: &dyn Query = g; Some(PlayerId(item.data[1] as u8)).into_iter().collect::<Vec<PlayerId>>() } { g.give_player_counters(player, planned_value!("counter kind Contract", CounterKind::Charge => CounterKind::Contract), amount); });
let had_5: Vec<_> = counters_5.iter().map(|c| g.state().obj(c.0).counters(c.1)).collect(); g.add_counters_together(&counters_5); let applied_5: Vec<ObjectId> = counters_5.iter().zip(had_5).filter(|(c, had)| g.state().obj(c.0).counters(c.1) > *had).map(|(c, _)| c.0).collect();
g.end_sentence();
planned!("duration-while-player-has-counter", CreateTrigger, "announcement" = "{\"modes\":(),\"targets\":[]}", "body" = "{\"instructions\":[{\"args\":{\"operation\":{\"args\":{\"entry\":{\"counters\":[],\"states\":[],\"tapped\":false},\"players\":{\"args\":{\"kind\":\"AbilityController\"},\"kind\":\"One\"},\"tokens\":[{\"amount\":{\"args\":1,\"kind\":\"Constant\"},\"recipe\":{\"CopyEach\":{\"adjustments\":[{\"args\":{\"family\":\"TokenCopyEffects\",\"form\":\"NoTokenCopyEffects\",\"operands\":[]},\"kind\":\"Term\"}],\"objects\":{\"args\":{\"args\":[{\"args\":[{\"args\":\"Artifact\",\"kind\":\"CardType\"},{\"args\":\"Creature\",\"kind\":\"CardType\"}],\"kind\":\"Either\"},{\"args\":{\"args\":{\"kind\":\"EventSubject\"},\"kind\":\"One\"},\"kind\":\"ControlledBy\"}],\"kind\":\"All\"},\"kind\":\"Battlefield\"}}},\"words\":\"CreatingEffect\"}]},\"kind\":\"CreateTokens\"},\"result\":3},\"kind\":\"Effect\"}]}", "duration" = "{\"WhilePlayerHasCounter\":{\"kind\":{\"Named\":\"ContractCounter\"},\"player\":{\"kind\":\"EventSubject\"}}}", "event" = "{\"Player\":{\"event\":\"LosesGame\",\"objects\":(),\"players\":{\"args\":{\"kind\":\"EventSubject\"},\"kind\":\"One\"}}}", "intervening_if" = "()", "once" = "false");
g.end_sentence();
hoisted_7.extend(applied_5.clone());
}
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Miss Highwater", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 0, 2, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Demon,Subtype::Advisor]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(5), toughness: Some(3), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Menace]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::DAMAGE.0), ability_1_event), ability_1, "Whenever Miss Highwater deals combat damage to a player who doesn't have a contract counter, they may discard their hand. If they do, they draw seven cards and get a contract counter. For as long as they have a contract counter, when they lose the game, for each artifact and creature they controlled, create a token that's a copy of it.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
