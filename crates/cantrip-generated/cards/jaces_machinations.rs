// Jace's Machinations
// status: planned
// requires: player rule for a while ActivationGrant
// Until end of turn, you may activate loyalty abilities of Jace planeswalkers you control on any player's turn any time you could cast an instant.
// Empower Jace 8. (Put eight loyalty counters on a Jace token you control. If you don't control one, first create a blue Jace planeswalker token with "[−1]: Surveil 1" and "[−3]: Draw a card.")
#[allow(unused_imports)] use cantrip_api::*;
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
planned!("player-rule-for-a-while-activation-grant" => if let Some(source) = { let q: &dyn Query = g; Some(asker.source) } { for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.add_player_effect(player, source, EffectKind::PlayerRule(PlayerRule::ActivationGrant { abilities: |q: &dyn Query, asker: &Asker, permanent: ObjectId, def: Option<&ActivatedDef>| (def.is_some_and(|d| d.is_loyalty_ability()) && (def.is_some_and(|d| d.from == ZoneKind::Battlefield) && (q.chars(permanent).has_subtype(q.words(asker.source).subtype(Subtype::Jace)) && q.chars(permanent).is(CardType::Planeswalker) && Some(asker.controller) == Some(q.controller(permanent))))), grant: ActivationGrant::InstantSpeedDuring(|q: &dyn Query, asker: &Asker, player: PlayerId| true) }), Duration::EndOfTurn); } });
g.end_sentence();
{ let player = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let amount = { let q: &dyn Query = g; (8i64).max(0) as i16 }; cantrip_mechanics::empower::jace(g, player, asker.source, amount)?; }
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Jace's Machinations", token_words: &[], oracle: true, cost: ManaCost { generic: 2, x: 0, symbols: [0, 1, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Instant]), subtypes: SubtypeSet::of(&[]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: spell(&[], ability_0), abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&cantrip_mechanics::empower::JACE];
