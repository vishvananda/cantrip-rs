// Underworld Rage-Hound
// status: ready
// This creature attacks each combat if able.
// Escape—{3}{R}, Exile three other cards from your graveyard. (You may cast this card from your graveyard for its escape cost.)
// This creature escapes with a +1/+1 counter on it.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_0_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.must_attack += 1; }
#[allow(unused_variables, unused_mut)] fn ability_2_escaped(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let asker = &Asker::new(entering.object, entering.controller); let more = { let q: &dyn Query = g; (1i64).max(0) as i16 }; entering.counters[CounterKind::PlusOne as usize] = entering.counters[CounterKind::PlusOne as usize].saturating_add(more);
 Ok(()) }
fn ability_2_enters(g: &mut dyn Game, entering: &mut Entering) -> Result<(), Suspend> { let escaped = { let q: &dyn Query = g; cantrip_mechanics::escape::escaped(q, entering.object) }; if escaped { ability_2_escaped(g, entering)?; } Ok(()) }

pub static CARD: CardDef = CardDef { name: "Underworld Rage-Hound", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Elemental,Subtype::Dog]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: None, power: Some(3), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[static_ability(Layer::Ability, cantrip_mechanics::statics::itself, ability_0_apply_0, "This creature attacks each combat if able.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: Some(ability_2_enters), modal: None, enchant: None, ways: &[cantrip_mechanics::escape::way(ManaCost { generic: 3, x: 0, symbols: [0, 0, 0, 1, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, 3, "Escape—{3}{R}, Exile three other cards from your graveyard.")], ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];
