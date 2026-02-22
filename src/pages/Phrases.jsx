import React, { useState, useEffect } from 'react';
import { motion } from 'framer-motion';
import { useStore } from '../components/StoreProvider';
import { invoke } from '@tauri-apps/api/core';

export default function Phrases() {
    const { settings, updateSettings } = useStore();
    const [isEnabled, setIsEnabled] = useState(settings?.phrases_enabled ?? true);
    const phrases = settings?.phrases || [];

    useEffect(() => {
        setIsEnabled(settings?.phrases_enabled ?? true);
    }, [settings?.phrases_enabled]);

    const handleToggle = async () => {
        const newState = !isEnabled;
        setIsEnabled(newState);
        await updateSettings({ phrases_enabled: newState });
        
        // 调用后端切换快捷键注册状态
        try {
            await invoke('toggle_phrase_shortcuts', { enabled: newState });
            console.log(`常用语快捷键已${newState ? '启用' : '禁用'}`);
        } catch (error) {
            console.error('切换快捷键状态失败:', error);
        }
    };

    return (
        <div className="h-full flex flex-col gap-6 p-6 ">
            {/* 常用语开关卡片 */}
            <motion.div
                className="w-full bg-white dark:bg-zinc-900 rounded-2xl p-6 border border-zinc-200 dark:border-zinc-800 shadow-[0_8px_30px_rgb(0,0,0,0.04)] dark:shadow-[0_8px_30px_rgb(0,0,0,0.1)] backdrop-blur-sm"
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
            >
                <div className="flex items-center justify-between">
                    <div>
                        <h1 className="text-2xl font-bold text-zinc-900 dark:text-white mb-2">常用语</h1>
                        <p className="text-sm text-zinc-600 dark:text-zinc-400">
                            {isEnabled ? '快捷键已启用' : '快捷键已禁用'}
                        </p>
                    </div>
                    <button
                        onClick={handleToggle}
                        className={`relative inline-flex h-8 w-16 items-center rounded-full transition-colors focus:outline-none ${
                            isEnabled
                                ? 'bg-green-500 hover:bg-green-600'
                                : 'bg-zinc-300 dark:bg-zinc-600 hover:bg-zinc-400 dark:hover:bg-zinc-500'
                        }`}
                    >
                        <span
                            className={`inline-block h-6 w-6 transform rounded-full bg-white transition-transform ${
                                isEnabled ? 'translate-x-9' : 'translate-x-1'
                            }`}
                        />
                    </button>
                </div>
            </motion.div>

            {/* 常用语表格卡片 */}
            <motion.div
                className={`w-full bg-white dark:bg-zinc-900 rounded-2xl p-6 border border-zinc-200 dark:border-zinc-800 shadow-[0_8px_30px_rgb(0,0,0,0.04)] dark:shadow-[0_8px_30px_rgb(0,0,0,0.1)] backdrop-blur-sm transition-opacity ${
                    !isEnabled ? 'opacity-50 pointer-events-none' : ''
                }`}
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ delay: 0.1 }}
            >
                <div className="overflow-auto">
                    <table className="min-w-full">
                        <thead>
                            <tr className="border-b border-zinc-200 dark:border-zinc-800">
                                <th className="py-3 text-left text-sm font-medium text-zinc-500 dark:text-zinc-400">
                                    文字
                                </th>
                                <th className="py-3 text-left text-sm font-medium text-zinc-500 dark:text-zinc-400">
                                    快捷键
                                </th>
                            </tr>
                        </thead>
                        <tbody className="divide-y divide-zinc-200 dark:divide-zinc-800">
                            {phrases.map((item) => (
                                <tr key={item.id} className="hover:bg-zinc-50 dark:hover:bg-zinc-800/50 transition-colors">
                                    <td className="py-4 text-zinc-900 dark:text-white">
                                        {item.phrase}
                                    </td>
                                    <td className="py-4">
                                        <span className="px-3 py-2 bg-zinc-100 dark:bg-zinc-800 rounded-lg text-base font-bold text-zinc-600 dark:text-zinc-300 shadow-sm hover:shadow-md transition-shadow border border-zinc-200 dark:border-zinc-700">
                                            {item.hotkey.shortcut}
                                        </span>
                                    </td>
                                </tr>
                            ))}
                        </tbody>
                    </table>
                </div>
            </motion.div>
            <div className="h-12 flex flex-col gap-6 p-6 ">

            </div>
        </div>
    );
} 