// Mutalith Vortex Beast
// status: ready
// Trample
// Warp Vortex — When this creature enters, flip a coin for each opponent you have. For each flip you win, draw a card. For each flip you lose, this creature deals 3 damage to that player.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>() };
let mut guard_0_0 = vec![false; players_0.len()];
let mut guard_0_1 = vec![false; players_0.len()];
let mut each_2 = vec![Default::default(); players_0.len()];
 for (index_0, _) in players_0.iter().copied().enumerate() { let player_1 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let flip_1 = g.flip_coin(player_1);
 each_2[index_0] = flip_1;
   } 
g.end_sentence();
 for (index_0, _) in players_0.iter().copied().enumerate() { if { let q: &dyn Query = g; each_2[index_0] } { guard_0_0[index_0] = true;
let amount_3 = { let q: &dyn Query = g; (1i64).max(0) as u32 }; let players_3 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_3 { g.draw(player, amount_3); }
   }
 } 
g.end_sentence();
let mut gathered_4: Vec<(ObjectId, Target, u16)> = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { if { let q: &dyn Query = g; !(each_2[index_0]) } { guard_0_1[index_0] = true;
let source_5 = { let q: &dyn Query = g; Some(asker.source) }; let amount_5 = { let q: &dyn Query = g; (3i64).max(0) as u16 }; if let Some(source) = source_5 { for t in { let q: &dyn Query = g; (Some(bound_0).into_iter().collect::<Vec<PlayerId>>()).into_iter().map(Target::Player).collect::<Vec<Target>>() } { gathered_4.push((source, t, amount_5)); } }
   }
 } 
g.damage_together(&gathered_4);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Mutalith Vortex Beast", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 1, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Mutant,Subtype::Beast]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(6), toughness: Some(6), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Trample]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_1_event), ability_1, "Warp Vortex — When this creature enters, flip a coin for each opponent you have. For each flip you win, draw a card. For each flip you lose, this creature deals 3 damage to that player.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
