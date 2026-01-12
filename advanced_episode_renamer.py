import requests
import json
import re
from typing import Dict, List, Optional
import os


class AdvancedEpisodeRenamer:
    def __init__(self, base_url: str, username: str, password: str):
        """
        高级剧集重命名工具
        
        :param base_url: OpenList服务的基础URL
        :param username: 用户名
        :param password: 密码
        """
        self.base_url = base_url.rstrip('/')
        self.username = username
        self.password = password
        self.token = None

    def login(self) -> bool:
        """
        登录获取JWT令牌
        """
        login_url = f"{self.base_url}/api/auth/login"

        payload = {
            "username": self.username,
            "password": self.password
        }

        try:
            response = requests.post(login_url, json=payload, timeout=30)
            response.raise_for_status()

            data = response.json()
            if data.get("code") == 200:
                self.token = data["data"]["token"]
                print("登录成功，获取到JWT令牌")
                return True
            else:
                print(f"登录失败: {data.get('message')}")
                return False

        except requests.exceptions.RequestException as e:
            print(f"登录请求失败: {e}")
            return False

    def get_directory_contents(self, path: str = "/") -> Optional[List[Dict]]:
        """
        获取目录内容
        """
        if not self.token:
            print("错误: 未登录，请先调用login方法")
            return None

        list_url = f"{self.base_url}/api/fs/list"

        payload = {
            "path": path
        }

        headers = {
            "Authorization": self.token,
            "Content-Type": "application/json"
        }

        try:
            response = requests.post(list_url, json=payload, headers=headers, timeout=30)
            response.raise_for_status()

            result = response.json()
            if result.get("code") == 200:
                return result.get("data", {}).get("content", [])
            else:
                print(f"获取目录内容失败: {result.get('message')}")
                return None

        except requests.exceptions.RequestException as e:
            print(f"获取目录内容请求失败: {e}")
            return None

    def batch_rename(self, src_dir: str, rename_mapping: Dict[str, str]) -> bool:
        """
        批量重命名文件
        
        :param src_dir: 源目录路径
        :param rename_mapping: 重命名映射字典，格式为 {原文件名: 新文件名}
        :return: 是否成功
        """
        if not self.token:
            print("错误: 未登录，请先调用login方法")
            return False

        rename_url = f"{self.base_url}/api/fs/batch_rename"

        # 构建重命名对象列表
        rename_objects = []
        for src_name, new_name in rename_mapping.items():
            rename_objects.append({
                "src_name": src_name,
                "new_name": new_name
            })

        payload = {
            "src_dir": src_dir,
            "rename_objects": rename_objects
        }

        headers = {
            "Authorization": self.token,
            "Content-Type": "application/json"
        }

        try:
            response = requests.post(rename_url, json=payload, headers=headers, timeout=60)
            response.raise_for_status()

            result = response.json()
            if result.get("code") == 200:
                print(f"批量重命名成功完成")
                print(f"处理了 {len(rename_objects)} 个文件")
                return True
            else:
                print(f"批量重命名失败: {result.get('message')}")
                return False

        except requests.exceptions.RequestException as e:
            print(f"批量重命名请求失败: {e}")
            return False

    def extract_episode_info(self, filename: str) -> Dict[str, str]:
        """
        从文件名中提取剧集信息
        
        :param filename: 原始文件名
        :return: 包含剧集信息的字典
        """
        # 常见的剧集文件名模式
        patterns = [
            r'(.+?)[\s._-]*S?(\d+)[\s._-]*E?(\d+)',  # S01E01 或 1x01 格式
            r'(.+?)[\s._-]*(\d+)[\s._-]*(\d{2})',  # 第1季第01集格式
            r'(.+?)[\s._-]*EP?[\s._-]*(\d+)',  # EP1 格式
            r'(.+?)[\s._-]*(\d+)[\s._-]*of[\s._-]*\d+',  # 1 of 10 格式
        ]

        for pattern in patterns:
            match = re.search(pattern, filename, re.IGNORECASE)
            if match:
                groups = match.groups()
                if len(groups) >= 2:
                    return {
                        'title': groups[0].strip(' ._-'),
                        'season': groups[1] if len(groups) > 1 else '1',
                        'episode': groups[2] if len(groups) > 2 else groups[1]
                    }

        # 如果没有匹配到模式，返回基本文件名信息
        name, ext = os.path.splitext(filename)
        return {
            'title': name,
            'season': '1',
            'episode': '1'
        }

    def generate_standard_name(self, episode_info: Dict[str, str],
                               naming_pattern: str = "S{season}E{episode:02d}_{title}") -> str:
        """
        根据剧集信息和命名模式生成标准文件名
        
        :param episode_info: 剧集信息字典
        :param naming_pattern: 命名模式
        :return: 标准文件名
        """
        try:
            season = str(episode_info.get('season', '1')).zfill(2)
            episode = int(episode_info.get('episode', '1'))
            title = episode_info.get('title', 'Unknown').strip()

            # 清理标题中的特殊字符
            title = re.sub(r'[<>:"/\\|?*]', '_', title)

            return naming_pattern.format(season=season, episode=episode, title=title)
        except Exception as e:
            print(f"生成标准名称时出错: {e}")
            return episode_info.get('title', 'Unknown')

    def create_smart_rename_mapping(self, directory_path: str,
                                    naming_pattern: str = "S{season}E{episode:02d}_{title}") -> Dict[str, str]:
        """
        智能创建重命名映射
        
        :param directory_path: 目录路径
        :param naming_pattern: 命名模式
        :return: 重命名映射字典
        """
        contents = self.get_directory_contents(directory_path)
        if not contents:
            return {}

        # 只处理视频文件
        video_extensions = {'.mp4', '.mkv', '.avi', '.mov', '.wmv', '.flv', '.webm', '.m4v', '.mpg', '.mpeg', '.ts',
                            '.m2ts', '.vob', '.iso'}
        rename_mapping = {}

        for item in contents:
            if item.get('is_dir', False):  # 跳过子目录
                continue

            filename = item.get('name', '')
            ext = os.path.splitext(filename)[1].lower()

            if ext in video_extensions:
                # 提取剧集信息
                episode_info = self.extract_episode_info(filename)

                # 添加文件扩展名到剧集信息
                episode_info['extension'] = ext

                # 生成新文件名
                new_name = self.generate_standard_name(episode_info, naming_pattern) + ext

                rename_mapping[filename] = new_name

        return rename_mapping

    def rename_episodes_by_pattern(self, directory_path: str, naming_pattern: str = "S{season}E{episode:02d}_{title}",
                                   dry_run: bool = False) -> bool:
        """
        根据模式重命名剧集
        
        :param directory_path: 目录路径
        :param naming_pattern: 命名模式
        :param dry_run: 是否为试运行模式（不实际执行重命名）
        :return: 是否成功
        """
        print(f"正在扫描目录: {directory_path}")

        # 创建重命名映射
        rename_mapping = self.create_smart_rename_mapping(directory_path, naming_pattern)

        if not rename_mapping:
            print("没有找到可重命名的视频文件")
            return False

        print(f"找到 {len(rename_mapping)} 个视频文件")

        # 显示重命名计划
        print("\n重命名计划:")
        for old_name, new_name in rename_mapping.items():
            print(f"  {old_name} -> {new_name}")

        if dry_run:
            print("\n这是试运行，不会实际执行重命名")
            return True

        # 执行批量重命名
        print(f"\n开始批量重命名...")
        return self.batch_rename(directory_path, rename_mapping)

    def rename_episodes_by_custom_list(self, directory_path: str, custom_mapping: Dict[str, str],
                                       dry_run: bool = False) -> bool:
        """
        根据自定义映射重命名剧集
        
        :param directory_path: 目录路径
        :param custom_mapping: 自定义重命名映射
        :param dry_run: 是否为试运行模式
        :return: 是否成功
        """
        if not custom_mapping:
            print("没有提供重命名映射")
            return False

        print(f"使用自定义映射重命名 {len(custom_mapping)} 个文件")

        # 显示重命名计划
        print("\n重命名计划:")
        for old_name, new_name in custom_mapping.items():
            print(f"  {old_name} -> {new_name}")

        if dry_run:
            print("\n这是试运行，不会实际执行重命名")
            return True

        # 执行批量重命名
        print(f"\n开始批量重命名...")
        return self.batch_rename(directory_path, custom_mapping)


def main():
    """
    使用示例
    """
    # 配置信息
    BASE_URL = "http://192.168.1.1:5244"  # 根据实际地址调整
    USERNAME = "admin"  # 替换为实际用户名
    PASSWORD = "147852369"  # 替换为实际密码

    # 创建重命名实例
    renamer = AdvancedEpisodeRenamer(BASE_URL, USERNAME, PASSWORD)

    # 登录
    if not renamer.login():
        print("无法登录到OpenList服务")
        return

    # 示例1: 智能重命名（自动识别剧集信息）
    print("=== 示例1: 智能重命名 ===")
    directory_path = "/BAIDU/影视/电视剧/剥茧"  # 替换为实际目录路径

    # 试运行，查看重命名计划
    renamer.rename_episodes_by_pattern(
        directory_path=directory_path,
        naming_pattern="{title}.S{season}E{episode:02d}",
        dry_run=True
    )

    # 如果试运行结果满意，执行实际重命名
    # renamer.rename_episodes_by_pattern(
    #     directory_path=directory_path,
    #     naming_pattern="S{season}E{episode:02d}_{title}",
    #     dry_run=False
    # )


if __name__ == "__main__":
    main()
