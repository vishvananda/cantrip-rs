// Maddening Hex
// status: planned
// requires: operation AttachEach
// Enchant player
// Whenever enchanted player casts a noncreature spell, roll a d6. This Aura deals damage to that player equal to the result. Then attach this Aura to another one of your opponents chosen at random.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_enchant(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Player(p) if true) }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.source).and_then(|o: ObjectId| q.state().obj(o).attached_player) == Some(*player) && !(q.chars(id).is(CardType::Creature)) { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let player_0 = { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }; let value_0 = i64::from(g.roll_die(player_0, 6).result);
g.end_sentence();
let source_1 = { let q: &dyn Query = g; Some(asker.source) }; let amount_1 = { let q: &dyn Query = g; (value_0).clamp(0, 65535) as u16 }; let recipients_1 = { let q: &dyn Query = g; (Some(PlayerId(item.data[0] as u8)).into_iter().collect::<Vec<PlayerId>>()).into_iter().map(Target::Player).collect::<Vec<Target>>() }; if let Some(source) = source_1 { let batch: Vec<(ObjectId, Target, u16)> = recipients_1.iter().map(|t| (source, *t, amount_1)).collect(); g.damage_together(&batch); }
g.end_sentence();
let pool_3: Vec<PlayerId> = { let q: &dyn Query = g; { let mut all = { let other = Some(asker.source).and_then(|o: ObjectId| q.state().obj(o).attached_player); q.state().apnap().filter(|p| Some(*p) != other).collect::<Vec<PlayerId>>() }; let other = Some(asker.controller).into_iter().flat_map(|p: PlayerId| q.state().opponents(p)).collect::<Vec<PlayerId>>(); all.retain(|p: &PlayerId| other.contains(p)); all } }; if pool_3.is_empty() { return Ok(()); } let chosen_3 = pool_3[g.choose_at_random(pool_3.len())];
g.end_sentence();
planned!("operation-attach-each" => cantrip_mechanics::attach::each(g, { let q: &dyn Query = g; Some(asker.controller).expect("a reference that always names a player") }, asker.source, { let q: &dyn Query = g; Some(asker.source).into_iter().collect::<Vec<ObjectId>>() }, { let q: &dyn Query = g; (Some(chosen_3).into_iter().collect::<Vec<PlayerId>>()).into_iter().map(Target::Player).collect::<Vec<Target>>() }, None, false)?);
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Maddening Hex", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 2, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Aura,Subtype::Curse]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::SPELL_CAST.0), ability_1_event), ability_1, "Whenever enchanted player casts a noncreature spell, roll a d6. This Aura deals damage to that player equal to the result. Then attach this Aura to another one of your opponents chosen at random.").targets(&[])], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: Some(target(ability_0_enchant)), ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
