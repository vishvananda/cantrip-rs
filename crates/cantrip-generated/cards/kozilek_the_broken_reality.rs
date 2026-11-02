// Kozilek, the Broken Reality
// status: planned
// requires: per-player Manifest
// When you cast this spell, up to two target players each manifest two cards from their hands. For each card manifested this way, you draw a card. (To manifest a card, put it onto the battlefield face down as a 2/2 creature. Turn it face up any time for its mana cost if it's a creature card.)
// Other colorless creatures you control get +3/+2.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Player(p) if true) }
static ABILITY_0_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_0_target_0).between(0, 2)];
#[allow(unused_variables)] fn ability_0_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && Some(asker.source) == Some(id) { return Some(Fired { subject, data }); } }
 None }
fn ability_0(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let mut players_0: Vec<PlayerId> = { let q: &dyn Query = g; q.state().apnap().filter(|&p| t_0.iter().any(|t| t.player() == Some(p))).collect::<Vec<PlayerId>>() };
planned!("per-player-manifest" => g.begin_batch(););
let mut each_4: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_4: Vec<ObjectId> = Vec::new();
let mut each_5: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_5: Vec<ObjectId> = Vec::new();
let mut each_6: Vec<Vec<ObjectId>> = vec![Vec::new(); players_0.len()]; let mut all_6: Vec<ObjectId> = Vec::new();
 for (index_0, bound_0) in players_0.iter().copied().enumerate() { let selected_2_player = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") }; let selected_2_candidates = { let q: &dyn Query = g; ((Some(bound_0).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::hand(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>() }; let selected_2_count = { let q: &dyn Query = g; cantrip_mechanics::select::Count::AsManyAsCan((2i64).max(0) as u32) }; let selected_2 = { cantrip_mechanics::choose::select(g, selected_2_player, asker.source, Purpose::Choose, selected_2_candidates, cantrip_mechanics::select::Selection::of(selected_2_count, &[]))? };
let controller_3 = { let q: &dyn Query = g; Some(bound_0).expect("a reference that always names a player") };
let cards_3 = { let q: &dyn Query = g; selected_2.clone() }; let entered_3: Vec<ObjectId> = g.put_all_onto_battlefield_face_down(&cards_3, controller_3, FaceDown::Manifested, false)?.into_iter().flatten().collect();
 each_4[index_0] = selected_2.clone(); all_4.extend(each_4[index_0].iter().copied());
each_5[index_0] = entered_3.clone(); all_5.extend(each_5[index_0].iter().copied());
each_6[index_0] = { let q: &dyn Query = g; cards_3.iter().copied().filter(|&o| q.state().obj(o).became.is_some()).collect::<Vec<ObjectId>>() }; all_6.extend(each_6[index_0].iter().copied());
   } 
planned!("per-player-manifest" => g.end_batch(););
g.end_sentence();
let amount_7 = { let q: &dyn Query = g; ((all_5.clone()).into_iter().count() as i64).max(0) as u32 }; let players_7 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
for player in players_7 { g.draw(player, amount_7); }
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_1_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (!(Some(asker.source) == Some(affected)) && ((*so_far).colors.is_empty() && (*so_far).is(CardType::Creature) && Some(asker.controller) == Some(q.controller(affected)))) }
#[allow(unused_variables)] fn ability_1_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); let (p, t) = ((3i64) as i16, (2i64) as i16); chars.power = chars.power.map(|v| v.saturating_add(p)); chars.toughness = chars.toughness.map(|v| v.saturating_add(t)); }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "Kozilek, the Broken Reality", token_words: &[], oracle: true, cost: ManaCost { generic: 9, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Eldrazi]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(9), toughness: Some(9), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::SPELL_CAST.0), ability_0_event), ability_0, "When you cast this spell, up to two target players each manifest two cards from their hands. For each card manifested this way, you draw a card.").targets(ABILITY_0_TARGETS).functions_in(ZoneKinds::of(&[ZoneKind::Stack]))], statics: &[static_ability(Layer::PtModify, ability_1_affects, ability_1_apply_0, "Other colorless creatures you control get +3/+2.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
