import requests
import json


class OpenListBulkRenamer:
    def __init__(self, base_url, username, password):
        """
        初始化OpenList批量重命名工具
        
        :param base_url: OpenList服务的基础URL
        :param username: 用户名
        :param password: 密码
        """
        self.base_url = base_url.rstrip('/')
        self.username = username
        self.password = password
        self.token = None

    def login(self):
        """
        登录获取JWT令牌
        """
        login_url = f"{self.base_url}/api/auth/login"
        
        payload = {
            "username": self.username,
            "password": self.password
        }
        
        try:
            response = requests.post(login_url, json=payload)
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

    def batch_rename(self, src_dir, rename_mapping):
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
            "Authorization": f"Bearer {self.token}",
            "Content-Type": "application/json"
        }
        
        try:
            response = requests.post(rename_url, json=payload, headers=headers)
            response.raise_for_status()
            
            result = response.json()
            if result.get("code") == 200:
                print(f"批量重命名成功完成")
                return True
            else:
                print(f"批量重命名失败: {result.get('message')}")
                return False
                
        except requests.exceptions.RequestException as e:
            print(f"批量重命名请求失败: {e}")
            return False

    def get_directory_contents(self, path="/"):
        """
        获取目录内容（可选功能，用于查看当前目录结构）
        """
        if not self.token:
            print("错误: 未登录，请先调用login方法")
            return None
            
        list_url = f"{self.base_url}/api/fs/list"
        
        payload = {
            "path": path
        }
        
        headers = {
            "Authorization": f"Bearer {self.token}",
            "Content-Type": "application/json"
        }
        
        try:
            response = requests.post(list_url, json=payload, headers=headers)
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


def main():
    """
    使用示例：批量重命名剧集
    """
    # 配置信息
    BASE_URL = "https://fox.oplist.org.cn"  # 根据实际地址调整
    USERNAME = "your_username"               # 替换为实际用户名
    PASSWORD = "your_password"               # 替换为实际密码
    
    # 要重命名的目录路径
    DIRECTORY_PATH = "/path/to/episodes"     # 替换为实际目录路径
    
    # 定义重命名映射关系 - 示例：将剧集从无序命名改为有序命名
    episode_mapping = {
        "episode_005.mkv": "S01E01_The_Beginning.mkv",
        "episode_001.mkv": "S01E02_Rising_Action.mkv",
        "episode_003.mkv": "S01E03_Midpoint_Crisis.mkv",
        "episode_002.mkv": "S01E04_Confrontation.mkv",
        "episode_004.mkv": "S01E05_Resolution.mkv",
        # 添加更多映射关系...
    }
    
    # 创建重命名实例
    renamer = OpenListBulkRenamer(BASE_URL, USERNAME, PASSWORD)
    
    # 登录
    if renamer.login():
        # 执行批量重命名
        success = renamer.batch_rename(DIRECTORY_PATH, episode_mapping)
        
        if success:
            print("所有剧集已成功重命名！")
        else:
            print("批量重命名过程中出现错误")
    else:
        print("无法登录到OpenList服务")


def create_custom_episode_mapping(directory_contents, naming_scheme="S{season:02d}E{episode:02d}_{title}"):
    """
    辅助函数：根据现有文件和命名方案创建重命名映射
    
    :param directory_contents: 目录中的文件列表
    :param naming_scheme: 命名方案模板
    :return: 重命名映射字典
    """
    import re
    
    mapping = {}
    
    # 提取所有视频文件
    video_extensions = ['.mp4', '.mkv', '.avi', '.mov', '.wmv', '.flv', '.webm', '.m4v', '.mpg', '.mpeg']
    video_files = []
    
    for item in directory_contents:
        if item.get('is_dir', False):  # 跳过子目录
            continue
            
        name = item.get('name', '')
        ext = name[name.rfind('.'):].lower()
        
        if ext in video_extensions:
            video_files.append(name)
    
    # 简单排序（按文件名）
    video_files.sort()
    
    # 为每个文件生成新的命名
    for idx, filename in enumerate(video_files, 1):
        # 获取文件扩展名
        ext_idx = filename.rfind('.')
        if ext_idx != -1:
            base_name = filename[:ext_idx]
            extension = filename[ext_idx:]
        else:
            base_name = filename
            extension = ''
        
        # 创建新名称
        new_filename = naming_scheme.format(
            season=1,  # 默认第一季
            episode=idx,
            title=f"Episode_{idx:02d}"
        ) + extension
        
        mapping[filename] = new_filename
    
    return mapping


if __name__ == "__main__":
    main()