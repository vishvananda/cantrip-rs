// The Warring Triad
// status: planned
// requires: engine: no mill cost
// Flying, trample, haste
// As long as there are fewer than eight cards in your graveyard, The Warring Triad isn't a creature.
// {T}, Mill a card: Target player adds one mana of any color. (Activate only as an instant.)
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_3_0_affects(q: &dyn LayerQuery, source: ObjectId, affected: ObjectId, so_far: &Chars) -> bool { let asker = &Asker::new(source, q.controller(source)); (((((Some(asker.controller).into_iter().collect::<Vec<PlayerId>>()).into_iter().flat_map(|p: PlayerId| q.state().zone(Zone::graveyard(p)).to_vec()).collect::<Vec<ObjectId>>()).into_iter().filter(|&id| true && true).collect::<Vec<ObjectId>>()).len() as i64) < (8i64)) && (source == affected) }
#[allow(unused_variables)] fn ability_3_0_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.types = TypeSet(chars.types.0 & !(1 << CardType::Creature as u8)); }
#[allow(unused_variables)] fn ability_4_target_0(q: &dyn Query, asker: &Asker, t: Target) -> bool { matches!(t, Target::Player(p) if true) }
static ABILITY_4_TARGETS: &[cantrip_api::TargetSpec] = &[target(ability_4_target_0)];
fn ability_4(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let t_0_raw = announced(g, item, 0, false); let t_0 = announced(g, item, 0, true);
let mana_players_1 = { let q: &dyn Query = g; t_0_raw.get(0).copied().filter(|t| t_0.contains(t)).and_then(|t: Target| t.player()).into_iter().collect::<Vec<PlayerId>>() };
let mana_1 = { let q: &dyn Query = g; (1i64).max(0) as u32 };
for player in mana_players_1 {
if mana_1 > 0 { let colors = cantrip_mechanics::choose_colors(g, player, asker.source, ColorSet(31), 1)?;
g.add_mana(player, colors[0].mana(), mana_1);}
}
g.end_sentence();
 Ok(()) }
fn announced(g: &dyn Game, item: &StackItem, spec: u8, legal: bool) -> Vec<Target> { (0..g.state().targets(item).len()).filter(|&i| g.state().spec(item, i) == spec && (!legal || g.target_legal(item, i))).map(|i| g.state().target(item, i)).collect() }

pub static CARD: CardDef = CardDef { name: "The Warring Triad", token_words: &[], oracle: true, cost: ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Artifact,CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::God]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(5), toughness: Some(5), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying,Keyword::Trample,Keyword::Haste]), protection: ColorSet(0), spell: None, abilities: &[ability(Cost::NONE.and(Cost::mana(ManaCost { generic: 0, x: 0, symbols: [0, 0, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] })).and(Cost { tap: true, ..Cost::NONE }).and(planned_value!("engine: no mill cost", Cost::NONE => Cost { mill: 1, ..Cost::NONE })), ability_4, "{T}, Mill a card: Target player adds one mana of any color.").targets(ABILITY_4_TARGETS)], statics: &[static_ability(Layer::Type, ability_3_0_affects, ability_3_0_apply_0, "As long as there are fewer than eight cards in your graveyard, The Warring Triad isn't a creature.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
