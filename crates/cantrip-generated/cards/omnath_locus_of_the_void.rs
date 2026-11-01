// Omnath, Locus of the Void
// status: planned
// requires: player rule UnspentManaBecomes
// Omnath gets +1/+1 for each unspent mana you have.
// If you would lose unspent mana, that mana becomes colorless instead.
// Landfall — Whenever a land you control enters, add {C}{C}.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let (p, t) = ((Some(asker.controller).map_or(0i64, |p: PlayerId| i64::from(q.state().player(p).pool.total() + q.state().player(p).lasting.total()))).clamp(-32768, 32767) as i16, (Some(asker.controller).map_or(0i64, |p: PlayerId| i64::from(q.state().player(p).pool.total() + q.state().player(p).lasting.total()))).clamp(-32768, 32767) as i16); chars.power = chars.power.map(|v| v.saturating_add(p)); chars.toughness = chars.toughness.map(|v| v.saturating_add(t)); }
#[allow(unused_variables)] fn ability_1_0_applies(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { player == asker.controller }
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::ZoneChange { old, new, from, to, .. } = event { let id = *new; let subject = Target::Object(id); let data = [0u32, u32::MAX]; if to.is(ZoneKind::Battlefield) && (q.chars(id).is(CardType::Land) && Some(asker.controller) == Some(q.controller(id))) { return Some(Fired { subject, data }); } }
 None }
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mana_players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_1_0 = { let q: &dyn Query = g; (2i64).max(0) as u32 };
for player in mana_players_1 {
g.add_mana(player, Mana::C, mana_1_0);}
g.end_sentence();
 Ok(()) }
planned_item!("player-rule-unspent-mana-becomes" => statics: player_rule(ability_1_0_applies, PlayerRule::UnspentManaBecomes(None)));

pub static CARD: CardDef = CardDef { name: "Omnath, Locus of the Void", token_words: &[], oracle: true, cost: ManaCost { generic: 7, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Elemental]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(6), toughness: Some(6), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ZONE_CHANGE.0), ability_2_event), ability_2, "Landfall — Whenever a land you control enters, add {C}{C}.").targets(&[])], statics: &[static_ability(Layer::PtModify, cantrip_mechanics::statics::itself, ability_0_apply_0, "Omnath gets +1/+1 for each unspent mana you have.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
