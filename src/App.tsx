// Copyright 2025 zl. All rights reserved.

import { useState, useCallback } from 'react'
import { open } from '@tauri-apps/plugin-dialog'
import { invoke } from '@tauri-apps/api/core'
import './App.css'

// ========================================
// 类型定义
// ========================================

interface CharacterDisplayInfo {
  // 基本信息
  name: string
  class: string
  class_en: string
  level: number

  // 状态
  hardcore: boolean
  died: boolean
  expansion: boolean

  // 难度进度
  normal_unlocked: boolean
  nightmare_unlocked: boolean
  hell_unlocked: boolean
  normal_act: number
  nightmare_act: number
  hell_act: number

  // 属性
  strength: number
  dexterity: number
  vitality: number
  energy: number
  unused_stat_points: number
  unused_skill_points: number

  // 生命/法力
  current_hp: number
  max_hp: number
  current_mana: number
  max_mana: number
  current_stamina: number
  max_stamina: number

  // 金币
  gold: number
  stash_gold: number

  // 经验
  experience: string

  // 时间戳
  created_at: number
  last_played: number
}

type TabType = 'info' | 'items' | 'skills' | 'quests'

// ========================================
// 物品类型 (与后端一致)
// ========================================

interface ItemDisplayInfo {
  id: string
  name: string
  base_item: string
  quality: string
  quality_en: string
  level: number
  identified: boolean
  ethereal: boolean
  socketed: boolean
  sockets: number
  quantity: number
  position: ItemPositionDisplay
  gridX?: number
  gridY?: number
  properties: ItemPropertyDisplay[]
}

interface ItemPropertyDisplay {
  name: string
  value: string
  min?: number
  max?: number
}

interface ItemPositionDisplay {
  location: string // "inventory", "equipment", "belt", "stash", "cube"
  slot?: string // "helm", "weapon", "ring_left", etc.
}

interface ItemsResponse {
  inventory: ItemDisplayInfo[]
  equipment: ItemDisplayInfo[]
  stash: ItemDisplayInfo[]
  cube: ItemDisplayInfo[]
  belt: ItemDisplayInfo[]
}

// ========================================
// 技能类型 (与后端一致)
// ========================================

interface SkillDisplayInfo {
  id: number
  name: string
  description: string
  level: number
  max_level: number
  skill_tree: string
  prerequisites: string[]
}

interface DifficultySkills {
  difficulty: string
  skills: SkillDisplayInfo[]
  available_points: number
}

interface SkillsResponse {
  class: string
  class_en: string
  normal: DifficultySkills
  nightmare: DifficultySkills
  hell: DifficultySkills
  total_available_points: number
}

// ========================================
// 任务类型 (与后端一致)
// ========================================

interface QuestDisplayInfo {
  id: string
  name: string
  act: number
  introduced: boolean
  completed: boolean
  reward_claimed: boolean
}

interface DifficultyQuests {
  difficulty: string
  act1: QuestDisplayInfo[]
  act2: QuestDisplayInfo[]
  act3: QuestDisplayInfo[]
  act4: QuestDisplayInfo[]
  act5: QuestDisplayInfo[]
}

interface QuestsResponse {
  normal: DifficultyQuests
  nightmare: DifficultyQuests
  hell: DifficultyQuests
}

// 装备槽位定义
const EQUIPMENT_SLOTS = [
  { id: 'helm', label: '头盔', gridArea: '2 / 2 / 3 / 3' },
  { id: 'amulet', label: '护身符', gridArea: '3 / 2 / 4 / 3' },
  { id: 'armor', label: '盔甲', gridArea: '4 / 2 / 5 / 3' },
  { id: 'weapon-main', label: '主手', gridArea: '3 / 1 / 4 / 2' },
  { id: 'weapon-off', label: '副手', gridArea: '3 / 3 / 4 / 4' },
  { id: 'gloves', label: '手套', gridArea: '4 / 1 / 5 / 2' },
  { id: 'belt', label: '腰带', gridArea: '5 / 2 / 6 / 3' },
  { id: 'ring-left', label: '左戒指', gridArea: '5 / 1 / 6 / 2' },
  { id: 'ring-right', label: '右戒指', gridArea: '5 / 3 / 6 / 4' },
  { id: 'boots', label: '鞋子', gridArea: '4 / 3 / 5 / 4' },
]

// 背包配置
const INVENTORY_ROWS = 8
const INVENTORY_COLS = 5

// ========================================
// 辅助函数
// ========================================

function getQualityColor(quality: string): string {
  const colors: Record<string, string> = {
    low: '#6b7280',
    normal: '#ffffff',
    superior: '#7dd3fc',
    magic: '#6666ff',
    set: '#00ff00',
    rare: '#ffff66',
    crafted: '#f0abfc',
    unique: '#c7b377',
  }
  // Handle both English and Chinese quality names
  return colors[quality] || colors[quality.toLowerCase()] || '#ffffff'
}

function getQualityLabel(quality: string): string {
  // If it's already in Chinese, return as is
  if (['劣质', '普通', '优越', '魔法', '套装', '稀有', '暗金'].includes(quality)) {
    return quality
  }
  // Otherwise map English to Chinese
  const labels: Record<string, string> = {
    low: '劣质',
    normal: '普通',
    superior: '优越',
    magic: '魔法',
    set: '套装',
    rare: '稀有',
    crafted: '手工',
    unique: '独特',
  }
  return labels[quality]
}

// ========================================
// 主组件
// ========================================

function App() {
  const [filePath, setFilePath] = useState<string>('')
  const [character, setCharacter] = useState<CharacterDisplayInfo | null>(null)
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string>('')
  const [activeTab, setActiveTab] = useState<TabType>('info')

  // 物品相关状态
  const [selectedItem, setSelectedItem] = useState<ItemDisplayInfo | null>(null)
  const [inventoryItems, setInventoryItems] = useState<ItemDisplayInfo[]>([])
  const [equipmentItems, setEquipmentItems] = useState<Record<string, ItemDisplayInfo>>({})

  // 技能相关状态
  const [skillsData, setSkillsData] = useState<SkillsResponse | null>(null)
  const [activeDifficulty, setActiveDifficulty] = useState<'normal' | 'nightmare' | 'hell'>('normal')

  // 任务相关状态
  const [questsData, setQuestsData] = useState<QuestsResponse | null>(null)
  const [activeQuestDifficulty, setActiveQuestDifficulty] = useState<'normal' | 'nightmare' | 'hell'>('normal')

  // 打开存档文件
  const handleOpenFile = async () => {
    setError('')
    try {
      const selected = await open({
        multiple: false,
        filters: [{
          name: 'D2S 存档文件',
          extensions: ['d2s']
        }]
      })

      if (selected && typeof selected === 'string') {
        setFilePath(selected)
        setLoading(true)

        // 并行加载角色信息、物品数据、技能数据和任务数据
        const [info, itemsData, skills, quests] = await Promise.all([
          invoke<CharacterDisplayInfo>('get_character_info', { filePath: selected }),
          invoke<ItemsResponse>('get_items', { filePath: selected }),
          invoke<SkillsResponse>('get_skills', { filePath: selected }),
          invoke<QuestsResponse>('get_quests', { filePath: selected })
        ])

        setCharacter(info)
        setInventoryItems(itemsData.inventory)
        setSkillsData(skills)
        setQuestsData(quests)

        // 将装备列表转换为按槽位索引的映射
        const equipMap: Record<string, ItemDisplayInfo> = {}
        for (const item of itemsData.equipment) {
          if (item.position.slot) {
            equipMap[item.position.slot] = item
          }
        }
        setEquipmentItems(equipMap)
      }
    } catch (e) {
      setError(e as string)
    } finally {
      setLoading(false)
    }
  }

  // 备份存档
  const handleBackup = async () => {
    if (!filePath) return
    try {
      const backupPath = await invoke<string>('backup_save', { filePath })
      alert(`存档已备份至:\n${backupPath}`)
    } catch (e) {
      setError(e as string)
    }
  }

  // 选择物品
  const handleSelectItem = useCallback((item: ItemDisplayInfo) => {
    setSelectedItem(item)
  }, [])

  // 获取难度名称
  const getDifficultyName = (act: number): string => {
    if (act >= 5) return '已完成'
    return `第 ${act + 1} 幕`
  }

  // 格式化时间戳
  const formatTimestamp = (ts: number): string => {
    if (!ts) return '-'
    const date = new Date(ts * 1000)
    return date.toLocaleString('zh-CN')
  }

  // 格式化数字
  const formatNumber = (num: number | string): string => {
    if (typeof num === 'string') {
      return parseInt(num).toLocaleString('zh-CN')
    }
    return num.toLocaleString('zh-CN')
  }

  // 标签页配置
  const tabs: { id: TabType; label: string; icon: string }[] = [
    { id: 'info', label: '基本信息', icon: '📋' },
    { id: 'items', label: '物品装备', icon: '🎒' },
    { id: 'skills', label: '技能', icon: '✨' },
    { id: 'quests', label: '任务', icon: '📜' },
  ]

  // 生成背包网格
  const renderInventoryGrid = () => {
    const cells = []
    for (let y = 0; y < INVENTORY_ROWS; y++) {
      for (let x = 0; x < INVENTORY_COLS; x++) {
        // 检查此位置是否有物品
        const item = inventoryItems.find(
          i => i.gridX !== undefined && i.gridX === x && i.gridY !== undefined && i.gridY === y
        )
        const width = 1 // TODO: 从物品基础数据获取
        const height = 1

        // 如果物品不是1x1，跳过已经被占用的格子
        if (item && (item.gridX !== x || item.gridY !== y)) {
          continue
        }

        cells.push(
          <div
            key={`${x}-${y}`}
            className={`grid-cell ${item ? 'occupied' : ''} ${selectedItem?.id === item?.id ? 'selected' : ''}`}
            style={
              item ? {
                gridColumn: `${x + 1} / span ${width}`,
                gridRow: `${y + 1} / span ${height}`,
              } : {}
            }
            onClick={() => item && handleSelectItem(item)}
            title={item ? `${getQualityLabel(item.quality)} ${item.name}` : '空'}
          >
            {item && (
              <div className="item-icon" style={{ borderColor: getQualityColor(item.quality_en) }}>
                <span className="item-name">{item.name}</span>
              </div>
            )}
          </div>
        )
      }
    }
    return cells
  }

  // 渲染装备槽位
  const renderEquipmentSlots = () => {
    return EQUIPMENT_SLOTS.map(slot => {
      const item = equipmentItems[slot.id]
      return (
        <div
          key={slot.id}
          className={`equip-slot ${selectedItem?.id === item?.id ? 'selected' : ''}`}
          style={{ gridArea: slot.gridArea }}
          onClick={() => item && handleSelectItem(item)}
          title={item ? `${getQualityLabel(item.quality)} ${item.name}` : slot.label}
        >
          {item ? (
            <div className="equip-item" style={{ borderColor: getQualityColor(item.quality_en) }}>
              <span className="equip-item-name">{item.name}</span>
            </div>
          ) : (
            <span className="slot-label">{slot.label}</span>
          )}
        </div>
      )
    })
  }

  // 渲染物品属性面板
  const renderPropertiesPanel = () => {
    if (!selectedItem) {
      return (
        <div className="properties-empty">
          <div className="empty-icon-small">📦</div>
          <p>选择一个物品查看属性</p>
        </div>
      )
    }

    return (
      <div className="properties-detail">
        {/* 物品名称和品质 */}
        <div className="item-header">
          <span
            className="item-quality-badge"
            style={{ backgroundColor: getQualityColor(selectedItem.quality_en) }}
          >
            {selectedItem.quality}
          </span>
          <h4 className="item-title" style={{ color: getQualityColor(selectedItem.quality_en) }}>
            {selectedItem.name}
          </h4>
          <p className="item-base">{selectedItem.base_item}</p>
        </div>

        {/* 状态标签 */}
        <div className="item-requirements">
          {selectedItem.identified && (
            <span className="req-badge req-identified">已鉴定</span>
          )}
          {selectedItem.ethereal && (
            <span className="req-badge req-ethereal">无形的</span>
          )}
          {selectedItem.socketed && selectedItem.sockets > 0 && (
            <span className="req-badge req-socketed">{selectedItem.sockets} 孔</span>
          )}
          {selectedItem.quantity > 1 && (
            <span className="req-badge req-quantity">数量: {selectedItem.quantity}</span>
          )}
        </div>

        {/* 属性列表 */}
        {selectedItem.properties.length > 0 && (
          <div className="item-properties">
            {selectedItem.properties.map((prop, i) => (
              <div key={i} className="property-line">
                <span className="property-name">{prop.name}:</span>
                <span className="property-value">{prop.value}</span>
              </div>
            ))}
          </div>
        )}

        {/* 暂无属性提示 */}
        {selectedItem.properties.length === 0 && (
          <div className="item-properties">
            <div className="property-line">
              <span className="property-value empty">暂无属性数据</span>
            </div>
          </div>
        )}

        {/* 操作按钮 */}
        <div className="item-actions">
          <button className="btn-small btn-edit" onClick={() => alert('编辑功能开发中...')}>
            ✏️ 编辑
          </button>
          <button className="btn-small btn-delete" onClick={() => alert('删除功能开发中...')}>
            🗑️ 删除
          </button>
        </div>
      </div>
    )
  }

  // 渲染技能列表
  const renderSkillsList = () => {
    if (!skillsData) return null

    // 获取当前难度的技能
    const difficultyData = skillsData[activeDifficulty as keyof SkillsResponse] as DifficultySkills
    const skills = difficultyData.skills

    // 按技能树分组
    const skillsByTree: Record<string, SkillDisplayInfo[]> = {}
    for (const skill of skills) {
      if (!skillsByTree[skill.skill_tree]) {
        skillsByTree[skill.skill_tree] = []
      }
      // 只显示有投资的技能 (level > 0) 或者显示所有技能但区分高亮
      skillsByTree[skill.skill_tree].push(skill)
    }

    // 处理技能等级变化
    const handleSkillLevelChange = async (skillId: number, newLevel: number) => {
      if (newLevel < 0 || newLevel > 20) return // 限制等级范围
      try {
        await invoke('set_skill_level', {
          filePath,
          difficulty: activeDifficulty,
          skillId,
          level: newLevel
        })
        // 重新加载技能数据
        const updated = await invoke<SkillsResponse>('get_skills', { filePath })
        setSkillsData(updated)
      } catch (e) {
        setError(e as string)
      }
    }

    return (
      <div className="skills-list-container">
        {Object.entries(skillsByTree).map(([treeName, treeSkills]) => (
          <div key={treeName} className="skill-tree-group">
            <h4 className="skill-tree-title">{treeName}</h4>
            <div className="skill-tree-content">
              {treeSkills.map(skill => (
                <div
                  key={skill.id}
                  className={`skill-item ${skill.level > 0 ? 'invested' : ''}`}
                >
                  <div className="skill-info">
                    <div className="skill-name">{skill.name}</div>
                    <div className="skill-description">{skill.description}</div>
                    {skill.prerequisites.length > 0 && (
                      <div className="skill-prerequisites">
                        前置: {skill.prerequisites.join(', ')}
                      </div>
                    )}
                  </div>
                  <div className="skill-level-control">
                    <button
                      className="skill-btn skill-btn-dec"
                      onClick={() => handleSkillLevelChange(skill.id, skill.level - 1)}
                      disabled={skill.level <= 0}
                      title="减少等级"
                    >
                      -
                    </button>
                    <div className="skill-level-display">
                      <span className={`skill-level ${skill.level > 0 ? 'has-points' : ''}`}>
                        {skill.level}
                      </span>
                      <span className="skill-max">/ {skill.max_level}</span>
                    </div>
                    <button
                      className="skill-btn skill-btn-inc"
                      onClick={() => handleSkillLevelChange(skill.id, skill.level + 1)}
                      disabled={skill.level >= skill.max_level || skillsData.total_available_points <= 0}
                      title="增加等级"
                    >
                      +
                    </button>
                  </div>
                </div>
              ))}
            </div>
          </div>
        ))}
      </div>
    )
  }

  // 渲染任务列表
  const renderQuestList = () => {
    if (!questsData) return null

    // 获取当前难度的任务
    const difficultyData = questsData[activeQuestDifficulty as keyof QuestsResponse] as DifficultyQuests

    // 幕配置
    const acts = [
      { key: 'act1', name: '第一幕 - 罗格营地', icon: '🏕️' },
      { key: 'act2', name: '第二幕 - 鲁高因', icon: '🏜️' },
      { key: 'act3', name: '第三幕 - 库拉斯特', icon: '🌴' },
      { key: 'act4', name: '第四幕 - 混沌避难所', icon: '🔥' },
      { key: 'act5', name: '第五幕 - 哈洛加斯', icon: '❄️' },
    ]

    // 处理任务状态切换
    const handleQuestToggle = async (quest: QuestDisplayInfo, field: 'introduced' | 'completed' | 'reward_claimed') => {
      try {
        await invoke('set_quest_status', {
          filePath,
          difficulty: activeQuestDifficulty,
          questId: quest.id,
          introduced: field === 'introduced' ? !quest.introduced : quest.introduced,
          completed: field === 'completed' ? !quest.completed : quest.completed,
          reward_claimed: field === 'reward_claimed' ? !quest.reward_claimed : quest.reward_claimed,
        })
        // 重新加载任务数据
        const updated = await invoke<QuestsResponse>('get_quests', { filePath })
        setQuestsData(updated)
      } catch (e) {
        setError(e as string)
      }
    }

    return (
      <div className="quests-list-container">
        {acts.map(act => {
          const actQuests = difficultyData[act.key as keyof DifficultyQuests] as QuestDisplayInfo[]
          if (actQuests.length === 0) return null

          return (
            <div key={act.key} className="quest-act-group">
              <h4 className="quest-act-title">
                <span className="act-icon">{act.icon}</span>
                <span>{act.name}</span>
              </h4>
              <div className="quest-act-content">
                {actQuests.map(quest => (
                  <div key={quest.id} className={`quest-item ${quest.completed ? 'completed' : ''}`}>
                    <div className="quest-info">
                      <div className="quest-name">{quest.name}</div>
                      <div className="quest-id">{quest.id}</div>
                    </div>
                    <div className="quest-status">
                      <label className="quest-checkbox">
                        <input
                          type="checkbox"
                          checked={quest.introduced}
                          onChange={() => handleQuestToggle(quest, 'introduced')}
                        />
                        <span>已接取</span>
                      </label>
                      <label className="quest-checkbox">
                        <input
                          type="checkbox"
                          checked={quest.completed}
                          onChange={() => handleQuestToggle(quest, 'completed')}
                        />
                        <span>已完成</span>
                      </label>
                      <label className="quest-checkbox">
                        <input
                          type="checkbox"
                          checked={quest.reward_claimed}
                          onChange={() => handleQuestToggle(quest, 'reward_claimed')}
                          disabled={!quest.completed}
                        />
                        <span>已领奖</span>
                      </label>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          )
        })}
      </div>
    )
  }

  return (
    <div className="app">
      {/* 顶部标题栏 */}
      <header className="app-header">
        <div className="header-left">
          <h1>⚔️ Diablo Tools</h1>
          <span className="version">v0.1.0</span>
        </div>
        <div className="header-right">
          <button onClick={handleOpenFile} disabled={loading} className="btn-open">
            {loading ? '加载中...' : '📂 打开存档'}
          </button>
          {character && (
            <button onClick={handleBackup} className="btn-backup">
              💾 备份
            </button>
          )}
        </div>
      </header>

      {/* 文件路径栏 */}
      {filePath && (
        <div className="file-bar">
          <span className="file-label">当前文件:</span>
          <span className="file-path" title={filePath}>{filePath.split('/').pop() || filePath}</span>
        </div>
      )}

      {/* 错误提示 */}
      {error && (
        <div className="error-message">❌ {error}</div>
      )}

      {/* 主内容区 */}
      <main className="app-main">
        {character ? (
          <>
            {/* 标签页导航 */}
            <nav className="tab-nav">
              {tabs.map(tab => (
                <button
                  key={tab.id}
                  className={`tab-btn ${activeTab === tab.id ? 'active' : ''}`}
                  onClick={() => setActiveTab(tab.id)}
                >
                  <span className="tab-icon">{tab.icon}</span>
                  <span className="tab-label">{tab.label}</span>
                </button>
              ))}
            </nav>

            {/* 标签页内容 */}
            <div className="tab-content">
              {activeTab === 'info' && (
                <div className="info-panel">
                  {/* 基本信息区域 */}
                  <section className="panel-section">
                    <h3>📋 角色信息</h3>
                    <div className="info-row">
                      <div className="info-group">
                        <label>角色名称</label>
                        <input type="text" value={character.name} readOnly />
                      </div>
                      <div className="info-group">
                        <label>职业</label>
                        <input type="text" value={character.class} readOnly />
                      </div>
                      <div className="info-group">
                        <label>等级</label>
                        <input type="number" value={character.level} readOnly />
                      </div>
                    </div>
                    <div className="info-row">
                      <div className="info-group">
                        <label>模式</label>
                        <div className={`badge ${character.hardcore ? 'hardcore' : 'softcore'}`}>
                          {character.hardcore ? '💀 专家模式' : '🛡️ 核心模式'}
                        </div>
                      </div>
                      <div className="info-group">
                        <label>状态</label>
                        <div className={`badge ${character.died ? 'dead' : 'alive'}`}>
                          {character.died ? '💀 已死亡' : '❤️ 存活'}
                        </div>
                      </div>
                      <div className="info-group">
                        <label>创建时间</label>
                        <span className="time-text">{formatTimestamp(character.created_at)}</span>
                      </div>
                    </div>
                  </section>

                  {/* 难度进度 */}
                  <section className="panel-section">
                    <h3>🗺️ 难度进度</h3>
                    <div className="difficulty-row">
                      {[
                        { name: '普通', icon: '⚔️', unlocked: character.normal_unlocked, act: character.normal_act },
                        { name: '噩梦', icon: '🔥', unlocked: character.nightmare_unlocked, act: character.nightmare_act },
                        { name: '地狱', icon: '💀', unlocked: character.hell_unlocked, act: character.hell_act },
                      ].map((diff, i) => (
                        <div key={i} className={`difficulty-item ${diff.unlocked ? 'unlocked' : 'locked'}`}>
                          <span className="diff-icon">{diff.icon}</span>
                          <span className="diff-name">{diff.name}</span>
                          <span className="diff-progress">{getDifficultyName(diff.act)}</span>
                        </div>
                      ))}
                    </div>
                  </section>

                  {/* 基础属性 */}
                  <section className="panel-section">
                    <h3>💪 基础属性</h3>
                    <div className="stats-grid">
                      <div className="stat-box">
                        <label>力量 (STR)</label>
                        <input type="number" value={character.strength} readOnly />
                      </div>
                      <div className="stat-box">
                        <label>敏捷 (DEX)</label>
                        <input type="number" value={character.dexterity} readOnly />
                      </div>
                      <div className="stat-box">
                        <label>体力 (VIT)</label>
                        <input type="number" value={character.vitality} readOnly />
                      </div>
                      <div className="stat-box">
                        <label>能量 (ENG)</label>
                        <input type="number" value={character.energy} readOnly />
                      </div>
                      <div className="stat-box highlight">
                        <label>可用属性点</label>
                        <input type="number" value={character.unused_stat_points} readOnly />
                      </div>
                      <div className="stat-box highlight">
                        <label>可用技能点</label>
                        <input type="number" value={character.unused_skill_points} readOnly />
                      </div>
                    </div>
                  </section>

                  {/* 生存属性 */}
                  <section className="panel-section">
                    <h3>❤️ 生存属性</h3>
                    <div className="stats-grid">
                      <div className="stat-bar-box">
                        <label>当前生命</label>
                        <div className="progress-bar">
                          <div className="progress-fill hp" style={{width: `${(character.current_hp / character.max_hp) * 100}%`}}></div>
                          <span className="progress-text">{character.current_hp} / {character.max_hp}</span>
                        </div>
                      </div>
                      <div className="stat-bar-box">
                        <label>当前法力</label>
                        <div className="progress-bar">
                          <div className="progress-fill mana" style={{width: `${(character.current_mana / character.max_mana) * 100}%`}}></div>
                          <span className="progress-text">{character.current_mana} / {character.max_mana}</span>
                        </div>
                      </div>
                      <div className="stat-bar-box">
                        <label>当前耐力</label>
                        <div className="progress-bar">
                          <div className="progress-fill stamina" style={{width: `${(character.current_stamina / character.max_stamina) * 100}%`}}></div>
                          <span className="progress-text">{character.current_stamina} / {character.max_stamina}</span>
                        </div>
                      </div>
                    </div>
                  </section>

                  {/* 金币 */}
                  <section className="panel-section">
                    <h3>💰 金币</h3>
                    <div className="gold-row">
                      <div className="gold-box">
                        <span className="gold-icon">🪙</span>
                        <div className="gold-content">
                          <span className="gold-label">身上金币</span>
                          <span className="gold-value">{formatNumber(character.gold)}</span>
                        </div>
                      </div>
                      <div className="gold-box">
                        <span className="gold-icon">🏦</span>
                        <div className="gold-content">
                          <span className="gold-label">仓库金币</span>
                          <span className="gold-value">{formatNumber(character.stash_gold)}</span>
                        </div>
                      </div>
                    </div>
                  </section>

                  {/* 经验值 */}
                  <section className="panel-section">
                    <h3>📊 经验值</h3>
                    <div className="exp-box">
                      <span className="exp-value">{formatNumber(character.experience)}</span>
                      <span className="exp-label">总经验值</span>
                    </div>
                  </section>
                </div>
              )}

              {activeTab === 'items' && (
                <div className="items-panel">
                  {/* 三栏布局：背包 | 装备 | 属性面板 */}
                  <div className="items-layout">
                    {/* 左侧：背包网格 */}
                    <div className="inventory-section">
                      <div className="section-header">
                        <h4>🎒 背包</h4>
                      </div>
                      <div className="inventory-grid">
                        {renderInventoryGrid()}
                      </div>
                    </div>

                    {/* 中间：装备显示区 */}
                    <div className="equipment-section">
                      <div className="section-header">
                        <h4>⚔️ 装备</h4>
                      </div>
                      <div className="equipment-grid">
                        {renderEquipmentSlots()}
                      </div>
                    </div>

                    {/* 右侧：属性面板 */}
                    <div className="properties-section">
                      <div className="section-header">
                        <h4>🔧 属性</h4>
                      </div>
                      <div className="properties-content">
                        {renderPropertiesPanel()}
                      </div>
                    </div>
                  </div>
                </div>
              )}

              {activeTab === 'skills' && (
                <div className="skills-panel">
                  {skillsData ? (
                    <>
                      {/* 技能头部信息 */}
                      <div className="skills-header">
                        <h2>{skillsData.class} 技能</h2>
                        <div className="skills-summary">
                          <span className="skills-points-badge">
                            可用技能点: <strong>{skillsData.total_available_points}</strong>
                          </span>
                        </div>
                      </div>

                      {/* 难度选择器 */}
                      <div className="difficulty-tabs">
                        {[
                          { key: 'normal', id: 'normal', label: '普通难度', icon: '⚔️' },
                          { key: 'nightmare', id: 'nightmare', label: '噩梦难度', icon: '🔥' },
                          { key: 'hell', id: 'hell', label: '地狱难度', icon: '💀' },
                        ].map(diff => (
                          <button
                            key={diff.id}
                            className={`diff-tab ${activeDifficulty === diff.id ? 'active' : ''}`}
                            onClick={() => setActiveDifficulty(diff.id as any)}
                          >
                            <span>{diff.icon}</span>
                            <span>{diff.label}</span>
                          </button>
                        ))}
                      </div>

                      {/* 技能列表 */}
                      <div className="skills-content">
                        {renderSkillsList()}
                      </div>
                    </>
                  ) : (
                    <div className="skills-empty">
                      <div className="empty-icon">✨</div>
                      <h3>技能系统</h3>
                      <p>请先打开存档以查看技能数据</p>
                    </div>
                  )}
                </div>
              )}

              {activeTab === 'quests' && (
                <div className="quests-panel">
                  {questsData ? (
                    <>
                      {/* 难度选择器 */}
                      <div className="difficulty-tabs">
                        {[
                          { key: 'normal', id: 'normal', label: '普通难度', icon: '⚔️' },
                          { key: 'nightmare', id: 'nightmare', label: '噩梦难度', icon: '🔥' },
                          { key: 'hell', id: 'hell', label: '地狱难度', icon: '💀' },
                        ].map(diff => (
                          <button
                            key={diff.id}
                            className={`diff-tab ${activeQuestDifficulty === diff.id ? 'active' : ''}`}
                            onClick={() => setActiveQuestDifficulty(diff.id as any)}
                          >
                            <span>{diff.icon}</span>
                            <span>{diff.label}</span>
                          </button>
                        ))}
                      </div>

                      {/* 任务列表 */}
                      <div className="quests-content">
                        {renderQuestList()}
                      </div>
                    </>
                  ) : (
                    <div className="quests-empty">
                      <div className="empty-icon">📜</div>
                      <h3>任务系统</h3>
                      <p>请先打开存档以查看任务数据</p>
                    </div>
                  )}
                </div>
              )}
            </div>
          </>
        ) : (
          /* 空状态 */
          <div className="empty-state">
            <div className="empty-icon">📜</div>
            <h3>未打开存档</h3>
            <p>点击右上角"打开存档"按钮选择 .d2s 文件</p>
            <p className="empty-hint">支持暗黑2重制版及术士君临版本</p>
          </div>
        )}
      </main>

      {/* 底部状态栏 */}
      <footer className="app-footer">
        <span className="footer-text">⚠️ 使用前请先备份存档 | 支持术士君临 (Reign of the Warlock)</span>
        {character && (
          <span className="footer-status">
            {character.class} · Lv.{character.level} · {character.name}
          </span>
        )}
      </footer>
    </div>
  )
}

export default App
