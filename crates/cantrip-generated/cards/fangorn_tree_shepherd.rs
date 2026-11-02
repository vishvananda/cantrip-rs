// Fangorn, Tree Shepherd
// status: planned
// requires: player rule RetainMana
// Treefolk you control have vigilance.
// Whenever one or more Treefolk you control attack, add twice that much {G}.
// You don't lose unspent green mana as steps and phases end.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); ((*so_far).has_subtype(q.words(asker.source).subtype(Subtype::Treefolk)) && Some(asker.controller) == Some(q.controller(affected))) }
#[allow(unused_variables)] fn ability_0_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.keywords = chars.keywords.with(Keyword::Vigilance); }
#[allow(unused_variables)] fn ability_1_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::Attacks { attacker, defender, defending, .. } = event { let id = *attacker; let subject = Target::Object(id); let data = [u32::from(q.controller(id).0), u32::from(defending.0)]; if (q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Treefolk)) && Some(asker.controller) == Some(q.controller(id))) && true { return Some(Fired { subject, data }); } }
 None }
fn ability_1(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let mana_players_1 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() };
let mana_1_0 = { let q: &dyn Query = g; ((2i64 * q.event_objects(item).len() as i64)).min(u32::MAX.into()) as u32 };
for player in mana_players_1 {
g.add_mana(player, Mana::G, mana_1_0);}
g.end_sentence();
 Ok(()) }
#[allow(unused_variables)] fn ability_2_0_applies(q: &dyn Query, asker: &Asker, player: PlayerId) -> bool { player == asker.controller }
planned_item!("player-rule-retain-mana" => statics: player_rule(ability_2_0_applies, PlayerRule::RetainMana(ColorSet(16))));

pub static CARD: CardDef = CardDef { name: "Fangorn, Tree Shepherd", token_words: &[], oracle: true, cost: ManaCost { generic: 4, x: 0, symbols: [0, 0, 0, 0, 3, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Treefolk]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(4), toughness: Some(10), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::ATTACKS.0), ability_1_event), ability_1, "Whenever one or more Treefolk you control attack, add twice that much {G}.").targets(&[]).once_per_batch()], statics: &[static_ability(Layer::Ability, ability_0_affects, ability_0_apply_0, "Treefolk you control have vigilance.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
