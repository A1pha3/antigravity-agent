import React from 'react';
import {motion} from 'framer-motion';
import {cn} from "@/utils/utils.ts";
import {Button, Space} from "antd";

interface UserSessionCardProps {
  nickName: string;
  userAvatar: string;
  email: string;
  // 0-1
  geminiQuota: number | -1;
  // 0-1
  claudeQuota: number | -1;
  // current
  isCurrentUser: boolean;
  onSelect: () => void
  onSwitch: () => void
  onDelete: () => void
}

export function AccountSessionListCard(props: UserSessionCardProps) {

  return (
    <motion.div
      onClick={props.onSelect}
      className={cn("w-[320px] rounded-2xl bg-white p-6 shadow-xl shadow-slate-200/60 border  border-slate-400 cursor-pointer", props.isCurrentUser && "")}
      initial={{opacity: 0, y: 20}}
      animate={{opacity: 1, y: 0}}
      transition={{duration: 0.5}}
    >
      {/* 头部区域 */}
      <header className="flex items-center gap-4 mb-4">
        <img
          src={props.userAvatar}
          className={cn(
            "h-12 w-12 rounded-full object-cover border-2 transition-all duration-300 flex-shrink-0 ring-2 ring-offset-2",
            props.isCurrentUser
              ? "border-blue-400 dark:border-blue-500 ring-blue-100 dark:ring-blue-900/50"
              : "border-gray-200 dark:border-gray-600 group-hover:border-blue-300 dark:group-hover:border-blue-500 ring-gray-100 dark:ring-gray-700/50 group-hover:ring-blue-100 dark:group-hover:ring-blue-900/30"
          )}
        />
        <div className="flex-1">
          <h2 className="text-lg font-bold text-slate-900 leading-tight">{props.nickName}</h2>
          <p className="text-sm text-slate-500 font-medium">{props.email}</p>
        </div>
        {
          props.isCurrentUser
            ? <div
              className="px-2.5 py-1 bg-green-100/80 text-green-700 text-xs font-bold rounded-md flex items-center gap-1">
              当前
            </div>
            : ""
        }
      </header>

      {/* 进度条区域 */}
      {
        props.geminiQuota === -1
          ? <div className="space-y-3">
            <UsageItem
              label="Gemini"
              percentage={-1}
              color="bg-blue-400"
              trackColor="bg-blue-50"
            />
            <UsageItem
              label="Claude"
              percentage={-1}
              color="bg-violet-400"
              trackColor="bg-violet-50"
            />
          </div>

          : <div className="space-y-3">
            <UsageItem
              label="Gemini"
              percentage={props.geminiQuota}
              color="bg-blue-400"
              trackColor="bg-blue-50"
            />
            <UsageItem
              label="Claude"
              percentage={props.claudeQuota}
              color="bg-violet-400"
              trackColor="bg-violet-50"
            />
          </div>
      }

      {/* 底部交互区域 */}
      <div className="mt-4 flex items-center justify-center relative">
        <Space.Compact>
          <Button
            onClick={e => {
              e.stopPropagation();
              props.onSwitch()
            }}
            disabled={props.isCurrentUser}
            type="primary"
          >
            切换
          </Button>
          <Button
            danger
            onClick={e => {
              e.stopPropagation()
              props.onDelete()
            }}
            disabled={props.isCurrentUser}
          >
            删除
          </Button>
        </Space.Compact>
      </div>
    </motion.div>
  );
}

// --- 子组件：进度条 ---
function UsageItem({label, percentage, color, trackColor}: {
  label: string,
  percentage: number,
  color: string,
  trackColor: string
}) {
  if (percentage === -1) {
    return <div className="group">
      <div className="flex justify-between mb-2 text-sm">
        <span className="text-slate-700 font-medium">{label}</span>
        <span className="text-slate-400 font-mono tabular-nums">Unknown</span>
      </div>
      <div className={cn("h-2.5 w-full rounded-full overflow-hidden", trackColor)}>
        <motion.div
          className={cn("h-full rounded-full", color)}
          initial={{width: 0}}
          animate={{width: `${percentage}%`}}
          transition={{type: "spring", stiffness: 40, damping: 12, delay: 0.2}}
        />
      </div>
    </div>
  }

  percentage = Math.round(percentage * 100);

  return (
    <div className="group">
      <div className="flex justify-between mb-2 text-sm">
        <span className="text-slate-700 font-medium">{label}</span>
        <span className="text-slate-400 font-mono tabular-nums">{percentage}%</span>
      </div>
      <div className={cn("h-2.5 w-full rounded-full overflow-hidden", trackColor)}>
        <motion.div
          className={cn("h-full rounded-full", color)}
          initial={{width: 0}}
          animate={{width: `${percentage}%`}}
          transition={{type: "spring", stiffness: 40, damping: 12, delay: 0.2}}
        />
      </div>
    </div>
  );
}
