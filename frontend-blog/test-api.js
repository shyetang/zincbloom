#!/usr/bin/env node

// 测试API连接
async function testApiConnection() {
  try {
    console.log("🔍 测试页面加载...");

    // 测试文章列表页面
    const pageResponse = await fetch("http://localhost:3000/posts");
    if (pageResponse.ok) {
      console.log("✅ 文章列表页面加载正常");
    }
    else {
      console.log("❌ 文章列表页面加载失败:", pageResponse.status);
    }

    // 测试文章列表
    console.log("\n🔍 测试文章列表API...");
    const postsResponse = await fetch("http://localhost:3000/api/blog/posts");
    const postsData = await postsResponse.json();

    console.log("✅ 文章列表API工作正常");
    console.log(`📝 找到 ${postsData.total_items} 篇文章`);

    if (postsData.items && postsData.items.length > 0) {
      const firstPost = postsData.items[0];
      console.log(`\n🔍 测试文章详情API: ${firstPost.slug}`);

      // 测试文章详情
      const postResponse = await fetch(`http://localhost:3000/api/blog/posts/${firstPost.slug}`);
      const postData = await postResponse.json();

      console.log("✅ 文章详情API工作正常");
      console.log(`📖 文章标题: ${postData.title}`);
      console.log(`📝 内容长度: ${postData.content_markdown?.length || 0} 字符`);

      // 测试文章详情页面
      console.log(`\n🔍 测试文章详情页面: /posts/${firstPost.slug}`);
      const postPageResponse = await fetch(`http://localhost:3000/posts/${firstPost.slug}`);
      if (postPageResponse.ok) {
        console.log("✅ 文章详情页面加载正常");
      }
      else {
        console.log("❌ 文章详情页面加载失败:", postPageResponse.status);
      }

      // 测试数据转换
      console.log("\n🔄 测试数据转换...");
      const hasMarkdown = postData.content_markdown ? "✅" : "❌";
      const hasHtml = postData.content_html ? "✅" : "❌";
      console.log(`Markdown 内容: ${hasMarkdown}`);
      console.log(`HTML 内容: ${hasHtml}`);

      console.log("\n🎉 所有测试通过！");
      console.log("\n📱 您现在可以在浏览器中访问：");
      console.log("   - 文章列表: http://localhost:3000/posts");
      console.log(`   - 文章详情: http://localhost:3000/posts/${firstPost.slug}`);
    }
    else {
      console.log("⚠️ 没有找到文章数据");
    }
  }
  catch (error) {
    console.error("❌ API测试失败:", error.message);
    process.exit(1);
  }
}

// 检查前端开发服务器是否运行
async function checkFrontendServer() {
  try {
    console.log("🔍 检查前端开发服务器...");
    const response = await fetch("http://localhost:3000");
    if (response.ok) {
      console.log("✅ 前端开发服务器运行正常");
      return true;
    }
    else {
      console.log("❌ 前端开发服务器响应异常");
      return false;
    }
  }
  catch (error) {
    console.log("❌ 前端开发服务器未运行 (端口3000)");
    console.log("💡 请运行: npm run dev");
    return false;
  }
}

// 检查后端服务器是否运行
async function checkBackendServer() {
  try {
    console.log("🔍 检查后端服务器...");
    const response = await fetch("http://localhost:8080/blog/posts");
    if (response.ok) {
      console.log("✅ 后端服务器运行正常");
      return true;
    }
    else {
      console.log("❌ 后端服务器响应异常");
      return false;
    }
  }
  catch (error) {
    console.log("❌ 后端服务器未运行 (端口8080)");
    console.log("💡 请确保后端服务器已启动");
    return false;
  }
}

// 主函数
async function main() {
  console.log("🚀 开始测试ZincBloom博客API集成...\n");

  // 检查服务器状态
  const frontendOk = await checkFrontendServer();
  const backendOk = await checkBackendServer();

  if (!frontendOk || !backendOk) {
    console.log("\n❌ 服务器检查失败，请确保前后端服务器都在运行");
    process.exit(1);
  }

  console.log("\n🔗 测试API集成和页面加载...");
  await testApiConnection();
}

main().catch(console.error);
