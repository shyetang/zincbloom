#!/usr/bin/env node

/**
 * 测试写作功能的简单脚本
 * 使用Node.js来测试API端点
 */

const https = require('http');

const API_BASE_URL = 'http://localhost:8080';

// 模拟的访问令牌（需要实际登录获取）
const ACCESS_TOKEN = process.env.ACCESS_TOKEN || 'your-access-token-here';

async function makeRequest(method, path, body = null) {
  return new Promise((resolve, reject) => {
    const options = {
      hostname: 'localhost',
      port: 8080,
      path: path,
      method: method,
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${ACCESS_TOKEN}`,
      }
    };

    const req = https.request(options, (res) => {
      let data = '';
      res.on('data', (chunk) => {
        data += chunk;
      });
      res.on('end', () => {
        try {
          const parsed = JSON.parse(data);
          resolve({ status: res.statusCode, data: parsed });
        } catch (e) {
          resolve({ status: res.statusCode, data: data });
        }
      });
    });

    req.on('error', (e) => {
      reject(e);
    });

    if (body) {
      req.write(JSON.stringify(body));
    }

    req.end();
  });
}

async function testWriteFeature() {
  console.log('🧪 开始测试写作功能...\n');

  try {
    // 1. 测试健康检查
    console.log('1. 测试后端健康状态...');
    const healthResponse = await makeRequest('GET', '/health');
    console.log(`   状态: ${healthResponse.status}`);
    console.log(`   响应: ${healthResponse.data}\n`);

    // 2. 测试用户统计API
    console.log('2. 测试用户统计API...');
    const statsResponse = await makeRequest('GET', '/me/stats');
    console.log(`   状态: ${statsResponse.status}`);
    console.log(`   响应:`, statsResponse.data, '\n');

    // 3. 测试创建文章
    console.log('3. 测试创建文章...');
    const createResponse = await makeRequest('POST', '/posts', {
      title: '测试文章',
      content_markdown: '# 这是一个测试文章\n\n这是测试内容。',
      status: 'draft'
    });
    console.log(`   状态: ${createResponse.status}`);
    console.log(`   响应:`, createResponse.data, '\n');

    if (createResponse.status === 200 && createResponse.data.id) {
      const postId = createResponse.data.id;

      // 4. 测试获取文章
      console.log('4. 测试获取文章...');
      const getResponse = await makeRequest('GET', `/posts/${postId}`);
      console.log(`   状态: ${getResponse.status}`);
      console.log(`   标题: ${getResponse.data?.title}\n`);

      // 5. 测试更新文章
      console.log('5. 测试更新文章...');
      const updateResponse = await makeRequest('PUT', `/posts/${postId}`, {
        title: '更新后的测试文章',
        content_markdown: '# 更新后的测试文章\n\n这是更新后的内容。',
        status: 'draft'
      });
      console.log(`   状态: ${updateResponse.status}`);
      console.log(`   响应:`, updateResponse.data, '\n');

      // 6. 清理 - 删除测试文章
      console.log('6. 清理测试数据...');
      const deleteResponse = await makeRequest('DELETE', `/posts/${postId}`);
      console.log(`   状态: ${deleteResponse.status}\n`);
    }

    // 7. 测试分类API
    console.log('7. 测试分类API...');
    const categoriesResponse = await makeRequest('GET', '/categories');
    console.log(`   状态: ${categoriesResponse.status}`);
    console.log(`   分类数量: ${categoriesResponse.data?.data?.length || 0}\n`);

    console.log('✅ 写作功能测试完成！');

  } catch (error) {
    console.error('❌ 测试失败:', error.message);
  }
}

// 检查是否提供了访问令牌
if (ACCESS_TOKEN === 'your-access-token-here') {
  console.log('⚠️  请设置 ACCESS_TOKEN 环境变量');
  console.log('   例如: ACCESS_TOKEN=your-token node test-write-feature.js');
  process.exit(1);
}

testWriteFeature();