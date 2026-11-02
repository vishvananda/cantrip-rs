// Namor the Sub-Mariner
// status: ready
// Flying
// Namor's power is equal to the number of Merfolk you control.
// Whenever you cast a noncreature spell with one or more blue mana symbols in its mana cost, create that many 1/1 blue Merfolk creature tokens.
#[allow(unused_imports)] use cantrip_api::*;
#[allow(unused_variables)] fn ability_1_apply_0(q: &dyn LayerQuery, source: ObjectId, chars: &mut Chars) { let asker = &Asker::new(source, q.controller(source)); chars.power = Some(((q.state().battlefield().iter().copied().filter(|&id| (q.chars(id).has_subtype(q.words(asker.source).subtype(Subtype::Merfolk)) && Some(asker.controller) == Some(q.controller(id)))).collect::<Vec<ObjectId>>()).len() as i64).clamp(-32768, 32767) as i16); }
#[allow(unused_variables)] fn ability_2_event(q: &dyn Query, asker: &Asker, event: &Event) -> Option<Fired> { if let Event::SpellCast { spell, player, .. } = event { let id = *spell; let subject = Target::Object(id); let data = [u32::from(player.0), u32::MAX]; if Some(asker.controller) == Some(*player) && (!(q.chars(id).is(CardType::Creature)) && { let cost = q.mana_cost(id); let m = Color::Blue.mana(); let i = m as usize; cost.symbols[i] > 0 || cost.twobrid.get(i).is_some_and(|&n| n > 0) || cost.phyrexian.get(i).is_some_and(|&n| n > 0) || cost.hybrid.iter().flatten().any(|&(a, b)| a == m || b == m) }) { return Some(Fired { subject, data }); } }
 None }
pub mod token_1 {
#[allow(unused_imports)] use cantrip_api::*;

pub static CARD: CardDef = CardDef { name: token!(Merfolk).name, token_words: token!(Merfolk).token_words, oracle: false, cost: ManaCost::ZERO, has_cost: false, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Merfolk]), supertypes: Supertypes { basic: false, legendary: false, snow: false }, colors: Some(ColorSet(2)), power: Some(1), toughness: Some(1), loyalty: None, defense: None, keywords: Keywords::of(&[]), protection: ColorSet(0), spell: None, abilities: &[], statics: &[], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[];

}
fn ability_2(g: &mut dyn Game, item: &StackItem) -> Result<(), Suspend> { let asker = &Asker::new(item.object, item.controller);
let players_0 = { let q: &dyn Query = g; Some(asker.controller).into_iter().collect::<Vec<PlayerId>>() }; let mut tokens_0 = Vec::new();
let count_0_0 = { let q: &dyn Query = g; ((item.subject.object().into_iter().collect::<Vec<ObjectId>>()).into_iter().map(|id: ObjectId| { let cost = q.mana_cost(id); let colors = [Color::Blue.mana()]; colors.iter().map(|&m| i64::from(cost.symbols[m as usize]) + i64::from(cost.phyrexian[m as usize]) + i64::from(cost.twobrid[m as usize])).sum::<i64>() + cost.hybrid.iter().flatten().filter(|(a, b)| colors.contains(a) || colors.contains(b)).count() as i64 }).sum::<i64>()).max(0) as u32 }; let def_0_0 = g.card_id(&token_1::CARD);
for player in players_0 {
tokens_0.extend(std::iter::repeat((def_0_0,player)).take(count_0_0 as usize));
} let words_0 = g.words(asker.source);
let created_0 = g.create_tokens_worded(&tokens_0, false, None, words_0)?;
g.end_sentence();
 Ok(()) }

pub static CARD: CardDef = CardDef { name: "Namor the Sub-Mariner", token_words: &[], oracle: true, cost: ManaCost { generic: 1, x: 0, symbols: [0, 2, 0, 0, 0, 0], hybrid: [None,None,None,None,None,None], twobrid: [0, 0, 0, 0, 0], phyrexian: [0, 0, 0, 0, 0] }, has_cost: true, types: TypeSet::of(&[CardType::Creature]), subtypes: SubtypeSet::of(&[Subtype::Mutant,Subtype::Merfolk,Subtype::Villain]), supertypes: Supertypes { basic: false, legendary: true, snow: false }, colors: None, power: Some(0), toughness: Some(4), loyalty: None, defense: None, keywords: Keywords::of(&[Keyword::Flying]), protection: ColorSet(0), spell: None, abilities: &[triggered(when_asked(EventMask(EventMask::SPELL_CAST.0), ability_2_event), ability_2, "Whenever you cast a noncreature spell with one or more blue mana symbols in its mana cost, create that many 1/1 blue Merfolk creature tokens.").targets(&[])], statics: &[characteristic_defining(Layer::PtDefine, ability_1_apply_0, "Namor's power is equal to the number of Merfolk you control.")], replacements: &[], preventions: &[], block_rules: &[], block_requirements: &[], attack_rules: &[], untap_bans: &[], delayed: &[], enters: None, modal: None, enchant: None, ..CardDef::BLANK };
pub static EXTRA: &[&CardDef] = &[&token_1::CARD];
