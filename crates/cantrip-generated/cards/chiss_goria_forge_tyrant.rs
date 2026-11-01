// Chiss-Goria, Forge Tyrant
// status: planned
// requires: operation PlayWith
// requires: player rule for a while SpecialAction
// Affinity for artifacts (This spell costs {1} less to cast for each artifact you control.)
// Flying, haste
// Whenever Chiss-Goria attacks, exile the top five cards of your library. You may cast an artifact spell from among them this turn. If you do, it has affinity for artifacts.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_affinity(q: &dyn Query, source: ObjectId, player: PlayerId, costed: Costed, targets: &[Target]) -> CostDelta { if costed != Costed::Spell(source) { return CostDelta::NONE; } let asker = &Asker::new(source, player); let count = q.permanents_of(player).into_iter().filter(|&id| q.chars(id).is(CardType::Artifact)).count(); CostDelta::less(count.min(255) as u8) }
#[allow(unused_variables)] fn ability_3_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(defending.0), u32::MAX]; if Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_3(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; (((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((5i64).max(0) as usize).collect::<Vec<ObjectId>>() }; let moved_0 = g.exile_all(&objects_0);
g.end_sentence();
planned!("player-rule-for-a-while-special-action" => if let Some(source) = { let q: &dyn Query = g; Some(asker.source) } { for player in { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() } { g.add_player_effect(player, source, EffectKind::PlayerRule(PlayerRule::SpecialAction { body: |g: &mut dyn Game, asker: &Asker| -> Result<(), Suspend> { planned!("operation-play-with" => cantrip_mechanics::play::with_terms(g, { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }, asker.source, CardPlay::CastSpell, { let q: &dyn Query = g; (moved_0.clone()).into_iter().filter(|&id| q.chars(id).is(CardType::Artifact)).collect::<Vec<ObjectId>>() }, { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((1i64).max(0) as u32) }, None, "{\"limit\":null,\"during\":null,\"cost\":\"Normal\",\"spend_as\":null,\"into_exile\":false,\"condition\":null,\"effects\":[{\"kind\":\"Term\",\"args\":{\"family\":\"SpellEffect\",\"form\":\"Affinity\",\"operands\":[{\"kind\":\"Objects\",\"args\":{\"kind\":\"Battlefield\",\"args\":{\"kind\":\"CardType\",\"args\":\"Artifact\"}}}]}}],\"flash\":false,\"may_look\":false,\"then\":null,\"face_down\":null}")?);
g.end_sentence();
 Ok(()) }, cost: None, limit: Some(PlayLimit::Total(1)), during: None, instant_speed: false }), Duration::EndOfTurn); } });
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Chiss-Goria, Forge Tyrant", token_words: &[], oracle: true, cost: ManaCost { generic: 6, x: 0, symbols: [0, 0, 0, 3, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Dragon]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(5), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::Haste]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_3_event), ability_3, "Whenever Chiss-Goria attacks, exile the top five cards of your library. You may cast an artifact spell from among them this turn. If you do, it has affinity for artifacts.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, own_cost_changes: &[ability_0_affinity], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
