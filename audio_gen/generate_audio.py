"""
BIOS Simulator - Bilingual Audio Asset Generator
Uses MiMo-V2.5-TTS-VoiceDesign via OpenAI-compatible API
"""

import json
import os
import sys
import base64
import time
from typing import Optional
from pathlib import Path

# Windows console UTF-8 support
if sys.platform == "win32":
    sys.stdout.reconfigure(encoding="utf-8", errors="replace")
    sys.stderr.reconfigure(encoding="utf-8", errors="replace")

from dotenv import load_dotenv
from openai import OpenAI

# Load .env from project root
load_dotenv(Path(__file__).parent.parent / ".env")

# ============ Configuration ============
API_KEY = os.environ.get("MIMO_API_KEY", "")
BASE_URL = os.environ.get("MIMO_API_URL", "https://token-plan-cn.xiaomimimo.com/v1/chat/completions")
MODEL = "mimo-v2.5-tts-voicedesign"
AUDIO_FORMAT = "wav"
MAX_RETRIES = 3
RETRY_DELAY = 3

# Paths
SCRIPT_DIR = Path(__file__).parent
SCRIPT_FILE = SCRIPT_DIR / "scripts" / "game_script.json"
OUTPUT_DIR = SCRIPT_DIR / "output"

# ============ Voice Profiles (VoiceDesign text descriptions) ============
VOICE_PROFILES_ZH = {
    "player": "年轻男性，二十五六岁，声音清澈干净，语速中等偏快，偶尔带有紧张感和自言自语的语气，像一个刚毕业的大学生",
    "grandfather": "老年男性，六十多岁，声音苍老但温暖，语速缓慢沉稳，偶尔带轻微咳嗽声，语气温慈祥，像一个慈祥的老爷爷在讲故事",
    "aunt_zhang": "中年女性，五十岁左右，声音热情亲切，语速适中，语气带着关心和些许八卦的好奇，像隔壁热心的邻居阿姨",
    "li_desheng": "中年男性，五十多岁，声音低沉威严，语速慢，语气不容置疑，带着隐隐的压迫感和商人的精明，像一个老练的企业高管",
    "narrator": "中性声音，平静客观，语速均匀，不带感情色彩，像纪录片的旁白解说员",
    "computer": "机械合成音，冷冰冰的电子音效，语速均匀无起伏，像90年代电脑的系统提示音",
    "env_monologue": "年轻男性，二十五六岁，声音压低、紧张不安，语速时快时慢，像在危险环境中小声自言自语，偶尔带有呼吸声和迟疑",
    "note": "中性声音，朗读信件或文件的语气，平静但带有一丝严肃，像在宣读一份重要的文档"
}

VOICE_PROFILES_EN = {
    "player": "Young male, mid-twenties, clear and clean voice with slight nervousness, medium-fast speaking pace, occasional self-mumbling tone, like a fresh college graduate",
    "grandfather": "Elderly male, sixties, aged but warm voice, slow and steady speaking pace, occasional gentle cough, kind and loving tone, like a gentle old grandfather telling a story",
    "aunt_zhang": "Middle-aged female, around fifty, warm and friendly voice, medium speaking pace, caring tone with a hint of curious gossip, like a kind neighbor aunt",
    "li_desheng": "Middle-aged male, fifties, deep and authoritative voice, slow speaking pace, tone of unquestionable authority with subtle menace and business shrewdness, like a seasoned corporate executive",
    "narrator": "Neutral voice, calm and objective, even speaking pace, no emotional coloring, like a documentary narrator",
    "computer": "Mechanical synthesized voice, cold electronic tone, even pace without inflection, like a 1990s computer system prompt",
    "env_monologue": "Young male, mid-twenties, hushed and tense voice, variable speaking pace, like whispering self-talk in a dangerous environment, occasional breathing and hesitation",
    "note": "Neutral voice, reading a letter or document tone, calm but with a hint of seriousness, like reading an important document aloud"
}

client = OpenAI(api_key=API_KEY, base_url=BASE_URL)


def load_script(script_path: Path) -> dict:
    with open(script_path, "r", encoding="utf-8") as f:
        return json.load(f)


def generate_audio(text: str, voice_description: str) -> bytes:
    """Call MiMo-V2.5-TTS-VoiceDesign via OpenAI SDK"""
    for attempt in range(1, MAX_RETRIES + 1):
        try:
            completion = client.chat.completions.create(
                model=MODEL,
                messages=[
                    {"role": "user", "content": voice_description},
                    {"role": "assistant", "content": text}
                ],
                audio={"format": AUDIO_FORMAT, "optimize_text_preview": True}
            )
            message = completion.choices[0].message
            return base64.b64decode(message.audio.data)

        except Exception as e:
            err_str = str(e)
            if "401" in err_str or "Invalid API Key" in err_str:
                print(f"    [AUTH ERROR] Invalid API Key. Set MIMO_API_KEY env var.")
                raise
            if "429" in err_str or "rate" in err_str.lower():
                wait = RETRY_DELAY * attempt * 2
                print(f"    [rate-limit] waiting {wait}s ({attempt}/{MAX_RETRIES})")
                time.sleep(wait)
                continue
            print(f"    [error] {e}")
            if attempt < MAX_RETRIES:
                time.sleep(RETRY_DELAY * attempt)
            else:
                raise

    raise RuntimeError(f"Failed after {MAX_RETRIES} retries")


def save_audio(audio_data: bytes, output_path: Path):
    output_path.parent.mkdir(parents=True, exist_ok=True)
    with open(output_path, "wb") as f:
        f.write(audio_data)


def process_segment(segment: dict, chapter_dir: Path, lang: str) -> dict:
    """Generate audio for one segment in one language"""
    seg_id = segment["id"]
    suffix = f"_{lang}"
    output_path = chapter_dir / f"{seg_id}{suffix}.{AUDIO_FORMAT}"

    if output_path.exists():
        return {"id": f"{seg_id}{suffix}", "status": "skipped", "path": str(output_path)}

    if lang == "zh":
        text = segment["text"]
        voice_desc = VOICE_PROFILES_ZH.get(segment["character"], VOICE_PROFILES_ZH["narrator"])
    else:
        text = segment.get("text_en", "")
        if not text:
            return {"id": f"{seg_id}{suffix}", "status": "skipped", "reason": "no English text"}
        voice_desc = VOICE_PROFILES_EN.get(segment["character"], VOICE_PROFILES_EN["narrator"])

    try:
        preview = text[:40].replace("\n", " ")
        print(f"  [{lang.upper()}] {seg_id} - {preview}...")
        audio_data = generate_audio(text, voice_desc)
        save_audio(audio_data, output_path)
        size_kb = len(audio_data) / 1024
        print(f"  [{lang.upper()}] {seg_id} done ({size_kb:.1f} KB)")
        return {"id": f"{seg_id}{suffix}", "status": "success", "path": str(output_path), "size": len(audio_data)}
    except Exception as e:
        print(f"  [{lang.upper()}] {seg_id} FAILED: {e}")
        return {"id": f"{seg_id}{suffix}", "status": "failed", "error": str(e)}


def generate_all(script_path: Path = SCRIPT_FILE, output_dir: Path = OUTPUT_DIR, langs: Optional[list] = None):
    """Generate all audio assets"""
    if langs is None:
        langs = ["zh", "en"]

    print("=" * 60)
    print("BIOS Simulator - Bilingual Audio Generator")
    print(f"Model:  {MODEL}")
    print(f"Format: {AUDIO_FORMAT}")
    print(f"Langs:  {', '.join(langs)}")
    print(f"Output: {output_dir}")
    print("=" * 60)

    script = load_script(script_path)
    chapters = script["chapters"]
    results = []

    for chapter_key, chapter in chapters.items():
        title_zh = chapter.get("title_zh", "")
        title_en = chapter.get("title", "")
        segments = chapter["segments"]
        chapter_dir = output_dir / chapter_key
        total = len(segments) * len(langs)

        print(f"\n{'─' * 50}")
        print(f"  {title_en}")
        print(f"  {title_zh} ({total} clips)")
        print(f"{'─' * 50}")

        for segment in segments:
            for lang in langs:
                result = process_segment(segment, chapter_dir, lang)
                results.append(result)
                time.sleep(0.5)

    print_summary(results, output_dir)
    return results


def generate_chapter(chapter_key: str, script_path: Path = SCRIPT_FILE, output_dir: Path = OUTPUT_DIR, langs: Optional[list] = None):
    """Generate audio for a specific chapter"""
    if langs is None:
        langs = ["zh", "en"]

    script = load_script(script_path)
    if chapter_key not in script["chapters"]:
        available = ", ".join(script["chapters"].keys())
        print(f"Error: Chapter '{chapter_key}' not found. Available: {available}")
        return

    chapter = script["chapters"][chapter_key]
    chapter_dir = output_dir / chapter_key
    results = []

    print(f"\n  {chapter.get('title', '')} / {chapter.get('title_zh', '')}")
    print("=" * 50)

    for segment in chapter["segments"]:
        for lang in langs:
            result = process_segment(segment, chapter_dir, lang)
            results.append(result)
            time.sleep(0.5)

    print_summary(results, output_dir)
    return results


def print_summary(results: list, output_dir: Path):
    total = len(results)
    success = sum(1 for r in results if r["status"] == "success")
    skipped = sum(1 for r in results if r["status"] == "skipped")
    failed = sum(1 for r in results if r["status"] == "failed")
    total_size = sum(r.get("size", 0) for r in results if r["status"] == "success")

    print(f"\n{'=' * 50}")
    print(f"  SUMMARY")
    print(f"{'=' * 50}")
    print(f"  Total:   {total}")
    print(f"  Success: {success}")
    print(f"  Skipped: {skipped}")
    print(f"  Failed:  {failed}")
    print(f"  Size:    {total_size / 1024 / 1024:.2f} MB")
    print(f"  Output:  {output_dir}")

    if failed > 0:
        print(f"\n  Failed:")
        for r in results:
            if r["status"] == "failed":
                print(f"    - {r['id']}: {r.get('error', 'unknown')}")


if __name__ == "__main__":
    if len(sys.argv) > 1:
        generate_chapter(sys.argv[1])
    else:
        generate_all()
