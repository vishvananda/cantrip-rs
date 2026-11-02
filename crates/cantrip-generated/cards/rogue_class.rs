// Rogue Class
// status: planned
// requires: play permission with mana as though any color
// (Gain the next level as a sorcery to add its ability.)
// Whenever a creature you control deals combat damage to a player, exile the top card of that player's library face down. You may look at it for as long as it remains exiled.
// {1}{U}{B}: Level 2
// Creatures you control have menace.
// {2}{U}{B}: Level 3
// You may play cards exiled with this Class, and you may spend mana as though it were mana of any color to cast those spells.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Damage { source, target, amount, kind, .. } = event { let (source, amount, combat) = (*source, *amount, kind.is_combat()); let subject = Target::Object(source); let data = [u32::from(amount), match target { Target::Player(p) => u32::from(p.0), _ => u32::MAX }]; if combat && !q.state().obj(source).zone.is(ZoneKind::Stack) && (q.chars(source).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(source))) && matches!(*target, Target::Player(p) if true) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let objects_0 = { let q: &dyn Query = g; (((Some(PlayerId(item.data[1] as u8)).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::library(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).into_iter().take((1i64).max(0) as usize).collect::<Vec<ObjectId>>() }; let by_0 = Some(g.linked(item)); let moved_0 = g.exile_face_down(&objects_0, by_0);
g.end_sentence();
let cards_1_0 = { let q: &dyn Query = g; (moved_0.clone()).get(0 as usize).copied().into_iter().collect::<Vec<ObjectId>>() }; let players_1_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; for &player in &players_1_0 { g.look_at(player, &cards_1_0); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_level_2_0_0_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (q.state().obj(source).class_level >= 2) && (((*so_far).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(affected)))) }
#[allow(unused_variables)] fn ability_1_level_2_0_0_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.keywords = chars.keywords.with(Keyword::Menace); }
planned_item!("play-permission-with-mana-as-though-any-color" => exile_casts: ExileCast { text: "Play from exile", allows: |q, asker, card, player| { player == asker.controller && (q.state().obj(asker.source).class_level >= 3) && q.exiled_with(q.linked_to(asker)).contains(&card) }, additional: Cost::NONE, any_type: false, lands: true, life_for_mana_value: false, any_color: true });

pub static CARD: CardDef = CardDef { name: "Rogue Class", token_words: &[], oracle: true, cost: ManaCost { generic: 0, x: 0, symbols: [0, 1, 1, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Enchantment]), subtypes: SubtypeSet::of(&[Subtype::Class]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: None, toughness: None, loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::DAMAGE.0), ability_0_event), ability_0, "Whenever a creature you control deals combat damage to a player, exile the top card of that player's library face down. You may look at it for as long as it remains exiled.").targets(&[]),cantrip_mechanics::class::level_two("{1}{U}{B}", "{1}{U}{B}: Level 2"),cantrip_mechanics::class::level_three("{2}{U}{B}", "{2}{U}{B}: Level 3")], statics: &[static_ability(Layer::Ability, ability_1_level_2_0_0_affects, ability_1_level_2_0_0_apply_0, "Creatures you control have menace.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
