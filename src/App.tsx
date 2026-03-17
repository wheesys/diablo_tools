// Copyright 2025 zl. All rights reserved.

import { useState } from 'react'
import { open } from '@tauri-apps/plugin-dialog'
import { invoke } from '@tauri-apps/api/core'
import './App.css'

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

function App() {
  const [filePath, setFilePath] = useState<string>('')
  const [character, setCharacter] = useState<CharacterDisplayInfo | null>(null)
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string>('')

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
        const info = await invoke<CharacterDisplayInfo>('get_character_info', {
          filePath: selected
        })
        setCharacter(info)
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

  return (
    <div className="app">
      <header className="app-header">
        <h1>⚔️ Diablo Tools</h1>
        <p className="subtitle">暗黑2重制版存档编辑器 - 支持术士君临</p>
      </header>

      <main className="app-main">
        {/* 文件操作栏 */}
        <div className="file-bar">
          <button onClick={handleOpenFile} disabled={loading}>
            {loading ? '加载中...' : '📂 打开存档'}
          </button>
          {character && (
            <button onClick={handleBackup} className="btn-backup">
              💾 备份存档
            </button>
          )}
          {filePath && (
            <span className="file-path" title={filePath}>{filePath.split('/').pop() || filePath}</span>
          )}
        </div>

        {error && (
          <div className="error-message">❌ {error}</div>
        )}

        {/* 角色信息面板 */}
        {character && (
          <div className="character-panel">
            {/* 基本信息卡片 */}
            <div className="panel-section">
              <h2>📋 角色信息</h2>
              <div className="info-grid">
                <div className="info-item">
                  <label>角色名称</label>
                  <span>{character.name || 'Unknown'}</span>
                </div>
                <div className="info-item">
                  <label>职业</label>
                  <span>{character.class}</span>
                </div>
                <div className="info-item">
                  <label>等级</label>
                  <span className="level-badge">{character.level}</span>
                </div>
                <div className="info-item">
                  <label>模式</label>
                  <span className={character.hardcore ? 'hardcore' : 'softcore'}>
                    {character.hardcore ? '💀 专家模式' : '🛡️ 核心模式'}
                  </span>
                </div>
                <div className="info-item">
                  <label>状态</label>
                  <span className={character.died ? 'dead' : 'alive'}>
                    {character.died ? '💀 已死亡' : '❤️ 存活'}
                  </span>
                </div>
                <div className="info-item">
                  <label>创建时间</label>
                  <span className="time-text">{formatTimestamp(character.created_at)}</span>
                </div>
              </div>
            </div>

            {/* 难度进度卡片 */}
            <div className="panel-section">
              <h2>🗺️ 难度进度</h2>
              <div className="difficulty-grid">
                <div className={`difficulty-card ${character.normal_unlocked ? 'unlocked' : 'locked'}`}>
                  <div className="difficulty-header">
                    <span className="difficulty-icon">⚔️</span>
                    <span className="difficulty-name">普通难度</span>
                  </div>
                  <div className="difficulty-progress">{getDifficultyName(character.normal_act)}</div>
                </div>
                <div className={`difficulty-card ${character.nightmare_unlocked ? 'unlocked' : 'locked'}`}>
                  <div className="difficulty-header">
                    <span className="difficulty-icon">🔥</span>
                    <span className="difficulty-name">噩梦难度</span>
                  </div>
                  <div className="difficulty-progress">{getDifficultyName(character.nightmare_act)}</div>
                </div>
                <div className={`difficulty-card ${character.hell_unlocked ? 'unlocked' : 'locked'}`}>
                  <div className="difficulty-header">
                    <span className="difficulty-icon">💀</span>
                    <span className="difficulty-name">地狱难度</span>
                  </div>
                  <div className="difficulty-progress">{getDifficultyName(character.hell_act)}</div>
                </div>
              </div>
            </div>

            {/* 基础属性卡片 */}
            <div className="panel-section">
              <h2>💪 基础属性</h2>
              <div className="stats-grid">
                <div className="stat-item">
                  <label>力量 (STR)</label>
                  <input type="number" value={character.strength} readOnly />
                  <span className="stat-name">Strength</span>
                </div>
                <div className="stat-item">
                  <label>敏捷 (DEX)</label>
                  <input type="number" value={character.dexterity} readOnly />
                  <span className="stat-name">Dexterity</span>
                </div>
                <div className="stat-item">
                  <label>体力 (VIT)</label>
                  <input type="number" value={character.vitality} readOnly />
                  <span className="stat-name">Vitality</span>
                </div>
                <div className="stat-item">
                  <label>能量 (ENG)</label>
                  <input type="number" value={character.energy} readOnly />
                  <span className="stat-name">Energy</span>
                </div>
                <div className="stat-item highlight">
                  <label>可用属性点</label>
                  <input type="number" value={character.unused_stat_points} readOnly />
                  <span className="stat-name">Stat Points</span>
                </div>
                <div className="stat-item highlight">
                  <label>可用技能点</label>
                  <input type="number" value={character.unused_skill_points} readOnly />
                  <span className="stat-name">Skill Points</span>
                </div>
              </div>
            </div>

            {/* 生存属性卡片 */}
            <div className="panel-section">
              <h2>❤️ 生存属性</h2>
              <div className="stats-grid">
                <div className="stat-item">
                  <label>当前生命</label>
                  <div className="stat-bar">
                    <div className="stat-bar-fill hp" style={{width: `${(character.current_hp / character.max_hp) * 100}%`}}></div>
                    <span className="stat-bar-text">{character.current_hp} / {character.max_hp}</span>
                  </div>
                </div>
                <div className="stat-item">
                  <label>当前法力</label>
                  <div className="stat-bar">
                    <div className="stat-bar-fill mana" style={{width: `${(character.current_mana / character.max_mana) * 100}%`}}></div>
                    <span className="stat-bar-text">{character.current_mana} / {character.max_mana}</span>
                  </div>
                </div>
                <div className="stat-item">
                  <label>当前耐力</label>
                  <div className="stat-bar">
                    <div className="stat-bar-fill stamina" style={{width: `${(character.current_stamina / character.max_stamina) * 100}%`}}></div>
                    <span className="stat-bar-text">{character.current_stamina} / {character.max_stamina}</span>
                  </div>
                </div>
              </div>
            </div>

            {/* 金币卡片 */}
            <div className="panel-section">
              <h2>💰 金币</h2>
              <div className="gold-section">
                <div className="gold-item">
                  <div className="gold-icon">🪙</div>
                  <div className="gold-info">
                    <span className="gold-label">身上金币</span>
                    <span className="gold-value">{formatNumber(character.gold)}</span>
                  </div>
                </div>
                <div className="gold-item">
                  <div className="gold-icon">🏦</div>
                  <div className="gold-info">
                    <span className="gold-label">仓库金币</span>
                    <span className="gold-value">{formatNumber(character.stash_gold)}</span>
                  </div>
                </div>
              </div>
            </div>

            {/* 经验值卡片 */}
            <div className="panel-section">
              <h2>📊 经验值</h2>
              <div className="exp-section">
                <span className="exp-value">{formatNumber(character.experience)}</span>
                <span className="exp-label">总经验值</span>
              </div>
            </div>

            {/* 操作栏 */}
            <div className="action-bar">
              <button onClick={() => alert('保存功能开发中...')} className="btn-primary">
                💾 保存修改
              </button>
            </div>
          </div>
        )}

        {/* 空状态 */}
        {!character && !loading && (
          <div className="empty-state">
            <div className="empty-icon">📜</div>
            <h3>未打开存档</h3>
            <p>点击上方"打开存档"按钮选择 .d2s 文件</p>
            <p className="empty-hint">支持暗黑2重制版及术士君临版本</p>
          </div>
        )}
      </main>

      <footer className="app-footer">
        <p>⚠️ 使用前请先备份存档 | 支持术士君临 (Reign of the Warlock)</p>
      </footer>
    </div>
  )
}

export default App
