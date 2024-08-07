export const buffOptionsMap = {
  1: '举步生风',
  2: '强力侧击',
  3: '怨恨',
  4: '觉醒',
  5: '身披重甲',
  6: '弱点压制',
  7: '以太充能',
  8: '妙手回春',
  9: '以太生成',
  10: '先发制人',
  11: '法力环绕',
  12: '法力充盈',
  13: '法力爆发',
  14: '护盾猛攻',
  15: '逃脱大师',
  16: '速战速决',
  17: '禁忌秘术',
  18: '重碾',
  19: '雷电之怒',
  20: '弱肉强食',
  21: '乘胜追击',
  22: '决斗大师',
  23: '迅捷利刃',
  24: '肾上腺素',
  25: '紧急救援',
  26: '迅猛突袭',
  27: '锐利眼神',
  28: '尖刺重锤',
  29: '精密短刀',
  30: '蓄力强化',
  31: '重量强化',
  32: '奇袭大师',
  33: '抗性护盾',
  34: '碎盾重锤',
  35: '反制冲击',
  36: '精力充沛',
  37: '爆炸专家',
  38: '破釜沉舟',
  39: '不屈',
  40: '回光返照',
  41: '坚韧意志',
  42: '女神庇佑',
  43: '咒术人偶',
}

export const buffOptions = [
  { value: '1', label: '举步生风' },
  { value: '2', label: '强力侧击' },
  { value: '3', label: '怨恨' },
  { value: '4', label: '觉醒' },
  { value: '5', label: '身披重甲' },
  { value: '6', label: '弱点压制' },
  { value: '7', label: '以太充能' },
  { value: '8', label: '妙手回春' },
  { value: '9', label: '以太生成' },
  { value: '10', label: '先发制人' },
  { value: '11', label: '法力环绕' },
  { value: '12', label: '法力充盈' },
  { value: '13', label: '法力爆发' },
  { value: '14', label: '护盾猛攻' },
  { value: '15', label: '逃脱大师' },
  { value: '16', label: '速战速决' },
  { value: '17', label: '禁忌秘术' },
  { value: '18', label: '重碾' },
  { value: '19', label: '雷电之怒' },
  { value: '20', label: '弱肉强食' },
  { value: '21', label: '乘胜追击' },
  { value: '22', label: '决斗大师' },
  { value: '23', label: '迅捷利刃' },
  { value: '24', label: '肾上腺素' },
  { value: '25', label: '紧急救援' },
  { value: '26', label: '迅猛突袭' },
  { value: '27', label: '锐利眼神' },
  { value: '28', label: '尖刺重锤' },
  { value: '29', label: '精密短刀' },
  { value: '30', label: '蓄力强化' },
  { value: '31', label: '重量强化' },
  { value: '32', label: '奇袭大师' },
  { value: '33', label: '抗性护盾' },
  { value: '34', label: '碎盾重锤' },
  { value: '35', label: '反制冲击' },
  { value: '36', label: '精力充沛' },
  { value: '37', label: '爆炸专家' },
  { value: '38', label: '破釜沉舟' },
  { value: '39', label: '不屈' },
  { value: '40', label: '回光返照' },
  { value: '41', label: '坚韧意志' },
  { value: '42', label: '女神庇佑' },
  { value: '43', label: '咒术人偶' },
]

export const debuffOptionsMap = {
  1: '减少防御力',
  2: '减少攻击力',
  3: '减少移动速度',
  4: '减少攻击速度',
}

export const debuffOptions = [
  { value: '1', label: '减少防御力' },
  { value: '2', label: '减少攻击力' },
  { value: '3', label: '减少移动速度' },
  { value: '4', label: '减少攻击速度' },
]

const classesWithBuffOptionsMap: Record<string, string> = {
  ...buffOptionsMap,
  44: '职业',
}

const classesWithBuffOptions = [
  ...buffOptions,
  { value: '44', label: '职业' },
]

const accessoryMap: Record<string, string> = {
  earring: '耳环',
  ring: '戒指',
  amulet: '项链',
}

export { classesWithBuffOptionsMap, classesWithBuffOptions, accessoryMap }
